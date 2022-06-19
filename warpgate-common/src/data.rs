use chrono::{DateTime, Utc};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warpgate_db_entities::Session;

use crate::{SessionId, Target, User};

#[derive(Serialize, Deserialize, Object)]
pub struct SessionSnapshot {
    pub id: SessionId,
    pub username: Option<String>,
    pub target: Option<Target>,
    pub started: DateTime<Utc>,
    pub ended: Option<DateTime<Utc>>,
    pub ticket_id: Option<Uuid>,
    pub protocol: String,
}

impl From<Session::Model> for SessionSnapshot {
    fn from(model: Session::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            target: model
                .target_snapshot
                .and_then(|s| serde_json::from_str(&s).ok()),
            started: model.started,
            ended: model.ended,
            ticket_id: model.ticket_id,
            protocol: model.protocol,
        }
    }
}

#[derive(Serialize, Deserialize, Object)]
pub struct UserSnapshot {
    pub username: String,
}

impl UserSnapshot {
    pub fn new(user: &User) -> Self {
        Self {
            username: user.username.clone(),
        }
    }
}
