use crate::battle_file;

pub type CardId = battle_file::CardId;
pub type LifeNumber = battle_file::LifeNumber;

#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    Me,
    Others,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CardAction {
    Damage { target: Target, amount: LifeNumber },
    Heal { target: Target, amount: LifeNumber },
}

#[derive(Debug, Clone)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub actions: Vec<CardAction>,
}
