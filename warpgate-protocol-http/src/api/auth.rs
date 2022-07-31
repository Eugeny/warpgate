use crate::common::{SessionExt, SessionAuthorization};
use crate::session::SessionStore;
use anyhow::Context;
use poem::session::Session;
use poem::web::Data;
use poem::Request;
use poem_openapi::payload::Json;
use poem_openapi::{ApiResponse, Enum, Object, OpenApi};
use warpgate_common::auth::{CredentialKind, AuthCredential};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::*;
use warpgate_common::{AuthResult, Secret, Services};

pub struct Api;

#[derive(Object)]
struct LoginRequest {
    username: String,
    password: String,
    otp: Option<String>,
}

#[derive(Enum)]
enum LoginFailureReason {
    InvalidCredentials,
    OtpNeeded,
    SsoNeeded,
}

#[derive(Object)]
struct LoginFailureResponse {
    reason: LoginFailureReason,
}

#[derive(ApiResponse)]
enum LoginResponse {
    #[oai(status = 201)]
    Success,

    #[oai(status = 401)]
    Failure(Json<LoginFailureResponse>),
}

#[derive(ApiResponse)]
enum LogoutResponse {
    #[oai(status = 201)]
    Success,
}

#[OpenApi]
impl Api {
    #[oai(path = "/auth/login", method = "post", operation_id = "login")]
    async fn api_auth_login(
        &self,
        req: &Request,
        session: &Session,
        services: Data<&Services>,
        session_middleware: Data<&Arc<Mutex<SessionStore>>>,
        body: Json<LoginRequest>,
    ) -> poem::Result<LoginResponse> {
        let mut credentials = vec![AuthCredential::Password(Secret::new(body.password.clone()))];
        if let Some(ref otp) = body.otp {
            credentials.push(AuthCredential::Otp(otp.clone().into()));
        }

        let result = {
            let mut config_provider = services.config_provider.lock().await;
            config_provider
                .authorize(&body.username, &credentials, crate::common::PROTOCOL_NAME)
                .await
                .context("Failed to authorize user")?
        };

        match result {
            AuthResult::Accepted { username } => {
                let server_handle = session_middleware
                    .lock()
                    .await
                    .create_handle_for(&req)
                    .await?;
                server_handle
                    .lock()
                    .await
                    .set_username(username.clone())
                    .await?;
                info!(%username, "Authenticated");
                session.set_auth(SessionAuthorization::User(username));
                Ok(LoginResponse::Success)
            }
            x => {
                error!("Auth rejected");
                Ok(LoginResponse::Failure(Json(LoginFailureResponse {
                    reason: match x {
                        AuthResult::Accepted { .. } => unreachable!(),
                        AuthResult::Rejected => LoginFailureReason::InvalidCredentials,
                        AuthResult::Need(CredentialKind::Otp) => LoginFailureReason::OtpNeeded,
                        AuthResult::Need(CredentialKind::Sso) => LoginFailureReason::SsoNeeded,
                        AuthResult::Need(kind) => {
                            error!("Unsupported required credential kind: {kind:?}");
                            LoginFailureReason::InvalidCredentials
                        }
                    },
                })))
            }
        }
    }

    #[oai(path = "/auth/logout", method = "post", operation_id = "logout")]
    async fn api_auth_logout(
        &self,
        session: &Session,
        session_middleware: Data<&Arc<Mutex<SessionStore>>>,
    ) -> poem::Result<LogoutResponse> {
        session_middleware.lock().await.remove_session(session);
        session.clear();
        info!("Logged out");
        Ok(LogoutResponse::Success)
    }
}
