use std::ops::{Sub, SubAssign};

use crate::*;

DeclareWrappedType!(CharacterId, id, u64);

pub enum CharacterRace {
    Human,
}

DeclareWrappedType!(Attack, damage, i64);

DeclareWrappedType!(Health, health, i64);

impl Sub<Attack> for Health {
    type Output = Self;

    fn sub(self, attack: Attack) -> Self {
        Health::new(self.health - attack.damage)
    }
}

impl SubAssign<Attack> for Health {
    fn sub_assign(&mut self, attack: Attack) {
        self.health -= attack.damage
    }
}

pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub race: CharacterRace,
    pub base_attack: Attack,

    pub health: Health,
}
