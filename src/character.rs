pub type CharacterId = u64;

pub enum CharacterRace {
    Human,
}

type Attack = i64;

pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub race: CharacterRace,
    pub base_attack: Attack,
}
