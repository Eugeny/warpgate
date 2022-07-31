use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::Secret;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CredentialKind {
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "publickey")]
    PublicKey,
    #[serde(rename = "otp")]
    Otp,
    #[serde(rename = "sso")]
    Sso,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthCredential {
    Otp(Secret<String>),
    Password(Secret<String>),
    PublicKey {
        kind: String,
        public_key_bytes: Bytes,
    },
    Sso {
        provider: String,
        email: String,
    },
}

impl AuthCredential {
    pub fn kind(&self) -> CredentialKind {
        match self {
            Self::Password { .. } => CredentialKind::Password,
            Self::PublicKey { .. } => CredentialKind::PublicKey,
            Self::Otp { .. } => CredentialKind::Otp,
            Self::Sso { .. } => CredentialKind::Sso,
        }
    }
}
