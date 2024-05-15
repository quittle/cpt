pub enum BattleHistory {
    Text(String),
    Id(String),
    Attack(String),
    Damage(String),
}

impl BattleHistory {
    pub fn text<S>(text: S) -> BattleHistory
    where
        S: Into<String>,
    {
        BattleHistory::Text(text.into())
    }

    pub fn id<S>(text: S) -> BattleHistory
    where
        S: Into<String>,
    {
        BattleHistory::Id(text.into())
    }

    pub fn attack<S>(text: S) -> BattleHistory
    where
        S: Into<String>,
    {
        BattleHistory::Attack(text.into())
    }

    pub fn damage<S>(text: S) -> BattleHistory
    where
        S: Into<String>,
    {
        BattleHistory::Damage(text.into())
    }
}
