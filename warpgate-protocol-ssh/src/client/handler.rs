use std::pin::Pin;

use crate::known_hosts::{KnownHostValidationResult, KnownHosts};
use crate::ConnectionError;
use futures::FutureExt;
use russh::client::Session;

use russh_keys::key::PublicKey;
use russh_keys::PublicKeyBase64;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use tracing::*;
use warpgate_common::{Services, TargetSSHOptions};

#[derive(Debug)]
pub enum ClientHandlerEvent {
    HostKeyReceived(PublicKey),
    HostKeyUnknown(PublicKey, oneshot::Sender<bool>),
    // ForwardedTCPIP(ChannelId, DirectTCPIPParams),
    Disconnect,
}

pub struct ClientHandler {
    pub ssh_options: TargetSSHOptions,
    pub event_tx: UnboundedSender<ClientHandlerEvent>,
    pub services: Services,
    pub session_tag: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientHandlerError {
    #[error("Connection error")]
    ConnectionError(ConnectionError),

    #[error("SSH")]
    Ssh(#[from] russh::Error),

    #[error("Internal error")]
    Internal,
}

impl russh::client::Handler for ClientHandler {
    type Error = ClientHandlerError;
    type FutureUnit = Pin<
        Box<dyn core::future::Future<Output = Result<(Self, Session), ClientHandlerError>> + Send>,
    >;
    type FutureBool = Pin<
        Box<dyn core::future::Future<Output = Result<(Self, bool), ClientHandlerError>> + Send>,
    >;

    fn finished_bool(self, b: bool) -> Self::FutureBool {
        async move { Ok((self, b)) }.boxed()
    }

    fn finished(self, session: Session) -> Self::FutureUnit {
        async move { Ok((self, session)) }.boxed()
    }

    fn check_server_key(self, server_public_key: &PublicKey) -> Self::FutureBool {
        let mut known_hosts = KnownHosts::new(&self.services.db);
        let server_public_key = server_public_key.clone();
        async move {
            self.event_tx
                .send(ClientHandlerEvent::HostKeyReceived(
                    server_public_key.clone(),
                ))
                .map_err(|_| ClientHandlerError::ConnectionError(ConnectionError::Internal))?;
            match known_hosts
                .validate(
                    &self.ssh_options.host,
                    self.ssh_options.port,
                    &server_public_key,
                )
                .await
            {
                Ok(KnownHostValidationResult::Valid) => Ok((self, true)),
                Ok(KnownHostValidationResult::Invalid {
                    key_type,
                    key_base64,
                }) => {
                    warn!(session=%self.session_tag, "Host key is invalid!");
                    return Err(ClientHandlerError::ConnectionError(
                        ConnectionError::HostKeyMismatch {
                            received_key_type: server_public_key.name().to_owned(),
                            received_key_base64: server_public_key.public_key_base64(),
                            known_key_type: key_type,
                            known_key_base64: key_base64,
                        },
                    ));
                }
                Ok(KnownHostValidationResult::Unknown) => {
                    warn!(session=%self.session_tag, "Host key is unknown");

                    let (tx, rx) = oneshot::channel();
                    self.event_tx
                        .send(ClientHandlerEvent::HostKeyUnknown(
                            server_public_key.clone(),
                            tx,
                        ))
                        .map_err(|_| ClientHandlerError::Internal)?;
                    let accepted = rx.await.map_err(|_| ClientHandlerError::Internal)?;
                    if accepted {
                        if let Err(error) = known_hosts
                            .trust(
                                &self.ssh_options.host,
                                self.ssh_options.port,
                                &server_public_key,
                            )
                            .await
                        {
                            error!(?error, session=%self.session_tag, "Failed to save host key");
                        }
                        Ok((self, true))
                    } else {
                        Ok((self, false))
                    }
                }
                Err(error) => {
                    error!(?error, session=%self.session_tag, "Failed to verify the host key");
                    Err(ClientHandlerError::Internal)
                }
            }
        }
        .boxed()
    }
}

impl Drop for ClientHandler {
    fn drop(&mut self) {
        let _ = self.event_tx.send(ClientHandlerEvent::Disconnect);
        debug!(session=%self.session_tag, "Dropped");
    }
}
