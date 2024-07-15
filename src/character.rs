use std::{
    cmp::min,
    ops::{Sub, SubAssign},
};

use serde::Serialize;

use crate::*;

type HandSize = battle_file::HandSize;

DeclareWrappedType!(CharacterId, id, usize);

#[derive(Serialize)]
pub enum CharacterRace {
    Human,
}

DeclareWrappedType!(Attack, damage, u64);

DeclareWrappedType!(Health, health, u64);

impl Sub<Attack> for Health {
    type Output = Self;

    fn sub(self, attack: Attack) -> Self {
        Health::new(self.health.saturating_sub(attack.damage))
    }
}

impl SubAssign<Attack> for Health {
    fn sub_assign(&mut self, attack: Attack) {
        self.health = (*self - attack).health;
    }
}

#[derive(Serialize)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub race: CharacterRace,
    pub hand: Vec<CardId>,
    pub deck: Vec<CardId>,
    pub health: Health,
    pub max_health: Health,
    pub remaining_actions: u8,
    pub hand_size: HandSize,
}

impl Character {
    pub fn is_dead(&self) -> bool {
        self.health.health == 0
    }

    pub fn reset_hand(&mut self, random_provider: &dyn RandomProvider) {
        self.hand = self
            .deck
            .pick_n_unique_linear(self.hand_size as usize, random_provider)
            .iter()
            .map(|v| **v)
            .collect();
    }

    pub fn get_default_turn_actions(&self) -> Option<u8> {
        None
    }

    pub fn heal(&mut self, healing: Health) {
        self.health = min(self.health + healing, self.max_health);
    }
}

#[derive(Clone)]
pub enum CharacterAction {
    Attack { name: String, base_damage: i64 },
}
