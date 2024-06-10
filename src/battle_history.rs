use std::fmt::Display;

use crate::TemplateEntry;

#[derive(Clone)]
pub enum BattleTextEntry {
    Id,
    Attack,
    Damage,
}

impl BattleTextEntry {
    pub fn id(text: &dyn Display) -> TemplateEntry<Self> {
        TemplateEntry::Typed(Self::Id, text.to_string())
    }

    pub fn attack(text: &dyn Display) -> TemplateEntry<Self> {
        TemplateEntry::Typed(Self::Attack, text.to_string())
    }

    pub fn damage(text: &dyn Display) -> TemplateEntry<Self> {
        TemplateEntry::Typed(Self::Damage, text.to_string())
    }
}

#[macro_export]
macro_rules! battle_markup {
    ( $( $x:expr ),* $(,)? ) => {
        {
            markup!(BattleTextEntry, $($x),*)
        }
    }
}

pub type BattleText = Vec<TemplateEntry<BattleTextEntry>>;
