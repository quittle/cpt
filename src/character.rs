use std::fmt::Display;

#[derive(PartialEq, Copy, Clone)]
pub struct CharacterId {
    id: u64,
}

impl CharacterId {
    pub const INVALID: Self = Self { id: std::u64::MAX };

    pub fn new(id: u64) -> Self {
        Self { id }
    }

    pub fn parse(id: &str) -> Option<Self> {
        Some(Self {
            id: id.parse::<u64>().ok()?,
        })
    }
}

impl Display for CharacterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.id))
    }
}

pub enum CharacterRace {
    Human,
}

pub type Attack = i64;
pub type Health = i64;

pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub race: CharacterRace,
    pub base_attack: Attack,

    pub health: Health,
}
