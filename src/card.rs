use serde::Serialize;

use crate::{battle_file, DeclareWrappedType};

DeclareWrappedType!(CardId, id, battle_file::CardId);

pub type LifeNumber = battle_file::LifeNumber;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Target {
    Me,
    Others,
    Any,
}

impl Target {
    /// Checks if `other` is compatible with `self`
    pub fn is_super_set(&self, other: &Self) -> bool {
        self == other || *self == Self::Any
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum CardAction {
    Damage { target: Target, amount: LifeNumber },
    Heal { target: Target, amount: LifeNumber },
}

impl CardAction {
    pub fn target(&self) -> &Target {
        match self {
            Self::Damage { target, .. } => target,
            Self::Heal { target, .. } => target,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub description: String,
    pub flavor: Option<String>,
    pub actions: Vec<CardAction>,
}

impl Card {
    /// If any action requires others, the target is Others
    /// If any action supports any and no target is others, the target is Any
    /// If neither are present, the target is Me
    pub fn target(&self) -> Target {
        let mut target = Target::Me;
        for action in &self.actions {
            match action.target() {
                Target::Others => return Target::Others,
                Target::Any => target = Target::Any,
                Target::Me => (),
            }
        }
        target
    }
}
