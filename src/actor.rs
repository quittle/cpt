use crate::*;

use async_trait::async_trait;

pub struct ActionRequest {
    pub description: String,
}

pub struct ActionFailure {
    pub message: String,
}

impl From<std::io::Error> for ActionFailure {
    fn from(value: std::io::Error) -> Self {
        ActionFailure {
            message: value.to_string(),
        }
    }
}

pub type ActionResult = Result<ActionRequest, ActionFailure>;

#[async_trait]
pub trait Actor {
    async fn act(&self, battle: &Battle, character_id: CharacterId) -> ActionResult;
    fn get_character_id(&self) -> CharacterId;
}
