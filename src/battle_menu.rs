use std::rc::Rc;

use crate::*;

pub enum BattleMenuOutput {
    Pass,
    Card { target: CharacterId, card: CardId },
}

type BattleMenuAction = MenuAction<Battle, BattleMenuOutput>;

pub type BattleMenu = Menu<Battle, BattleMenuOutput>;

pub struct CardSelectionItem {
    target: CharacterId,
    card: CardId,
}

impl MenuItem<Battle, BattleMenuOutput> for CardSelectionItem {
    fn label(&self, battle: &Battle) -> String {
        battle.characters[&self.target].name.clone()
    }

    fn action(&self, _battle: &Battle) -> BattleMenuAction {
        MenuAction::Done(BattleMenuOutput::Card {
            target: self.target,
            card: self.card,
        })
    }
}

pub struct ActionsMenu {
    pub me: CharacterId,
    pub cards: Vec<CardId>,
    pub targets: Vec<CharacterId>,
}

impl MenuItem<Battle, BattleMenuOutput> for ActionsMenu {
    fn label(&self, _battle: &Battle) -> String {
        "Cards".to_string()
    }

    fn action(&self, _battle: &Battle) -> BattleMenuAction {
        BattleMenuAction::MenuItem(
            self.cards
                .iter()
                .map(|card| -> Rc<dyn MenuItem<Battle, BattleMenuOutput>> {
                    Rc::new(CardMenu {
                        me: self.me,
                        card: *card,
                        targets: self.targets.clone(),
                    })
                })
                .collect(),
        )
    }
}

pub struct CardMenu {
    pub me: CharacterId,
    pub card: CardId,
    pub targets: Vec<CharacterId>,
}

impl MenuItem<Battle, BattleMenuOutput> for CardMenu {
    fn label(&self, battle: &Battle) -> String {
        battle.cards[&self.card].name.clone()
    }

    fn action(&self, battle: &Battle) -> BattleMenuAction {
        match battle.cards[&self.card].target() {
            Target::Me => MenuAction::Done(BattleMenuOutput::Card {
                target: self.me,
                card: self.card,
            }),
            Target::Others => MenuAction::MenuItem(
                self.targets
                    .iter()
                    .map(|target| -> Rc<dyn MenuItem<Battle, BattleMenuOutput>> {
                        Rc::new(CardSelectionItem {
                            target: *target,
                            card: self.card,
                        })
                    })
                    .collect(),
            ),
        }
    }
}

pub struct PassMenuItem {}

impl MenuItem<Battle, BattleMenuOutput> for PassMenuItem {
    fn label(&self, _battle: &Battle) -> String {
        "Pass".to_string()
    }

    fn action(&self, _battle: &Battle) -> BattleMenuAction {
        MenuAction::Done(BattleMenuOutput::Pass)
    }
}
