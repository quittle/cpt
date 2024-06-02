use std::ops::{Sub, SubAssign};

use crate::*;

type HandSize = battle_file::HandSize;

DeclareWrappedType!(CharacterId, id, usize);

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
    pub hand: Vec<CardId>,
    pub deck: Vec<CardId>,
    pub health: Health,
    pub hand_size: HandSize,
}

impl Character {
    pub fn is_dead(&self) -> bool {
        self.health.health <= 0
    }

    pub fn reset_hand(&mut self, random_provider: &dyn RandomProvider) {
        self.hand = self
            .deck
            .pick_n_unique_linear(self.hand_size as usize, random_provider)
            .iter()
            .map(|v| **v)
            .collect();
    }
}

#[derive(Clone)]
pub enum CharacterAction {
    Attack { name: String, base_damage: i64 },
}
