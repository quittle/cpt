use crate::*;

use async_trait::async_trait;

pub enum Action {
    Pass,
    AttackCharacter(CharacterId, Attack),
}

pub struct ActionRequest {
    pub description: String,
    pub action: Action,
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
pub trait Actor: Sync {
    async fn act(&self, battle: &Battle, character_id: CharacterId) -> ActionResult;
    fn damage(&mut self, _attacker: CharacterId, damage: Attack) {
        self.get_mut_character().health -= damage;
    }

    fn get_character(&self) -> &Character;
    fn get_mut_character(&mut self) -> &mut Character;
}
