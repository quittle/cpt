use std::fmt::Display;

pub enum BattleHistory {
    Text(String),
    Id(String),
    Attack(String),
    Damage(String),
}

impl BattleHistory {
    pub fn text(text: &dyn Display) -> BattleHistory {
        BattleHistory::Text(text.to_string())
    }

    pub fn id(text: &dyn Display) -> BattleHistory {
        BattleHistory::Id(text.to_string())
    }

    pub fn attack(text: &dyn Display) -> BattleHistory {
        BattleHistory::Attack(text.to_string())
    }

    pub fn damage(text: &dyn Display) -> BattleHistory {
        BattleHistory::Damage(text.to_string())
    }
}
