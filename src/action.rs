use crate::*;

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
