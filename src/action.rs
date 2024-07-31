use std::{fmt::Display, process::ExitCode};

use crate::*;

#[derive(Debug)]
pub enum Action {
    Pass,
    Act(CardId, CharacterId),
    Move(CharacterId, GridLocation),
}

#[derive(Debug)]
pub struct ActionFailure {
    pub message: String,
}

#[derive(Debug)]
pub enum ActionError {
    Failure(ActionFailure),
    Exit(ExitCode),
}

impl ActionError {
    pub fn fail(message: impl Into<String>) -> ActionError {
        Self::Failure(ActionFailure {
            message: Into::into(message),
        })
    }
}

impl From<std::io::Error> for ActionError {
    fn from(value: std::io::Error) -> Self {
        Self::Failure(ActionFailure {
            message: value.to_string(),
        })
    }
}

impl Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Failure(failure) => f.write_str(&failure.message),
            Self::Exit(code) => write!(f, "Exit Code {:?}", code),
        }
    }
}

pub type ActionResult = Result<Action, ActionError>;
