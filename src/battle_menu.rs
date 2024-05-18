use std::rc::Rc;

use crate::*;

pub enum BattleMenuOutput {
    Pass,
    Card {
        target: String,
        card_name: String,
        base_attack: Attack,
    },
}

type BattleMenuAction = MenuAction<BattleMenuOutput>;

pub type BattleMenu = Menu<BattleMenuOutput>;

pub struct CardSelectionItem {
    target: String,
    card_name: String,
    base_attack: Attack,
}

impl MenuItem<BattleMenuOutput> for CardSelectionItem {
    fn label(&self) -> &str {
        &self.target
    }

    fn action(&self) -> BattleMenuAction {
        MenuAction::Done(BattleMenuOutput::Card {
            target: self.target.clone(),
            card_name: self.card_name.clone(),
            base_attack: self.base_attack,
        })
    }
}

pub struct ActionsMenu {
    pub actions: Vec<CharacterAction>,
    pub targets: Vec<String>,
}

impl MenuItem<BattleMenuOutput> for ActionsMenu {
    fn label(&self) -> &str {
        "Cards"
    }

    fn action(&self) -> BattleMenuAction {
        BattleMenuAction::MenuItem(
            self.actions
                .iter()
                .map(|action| -> Rc<dyn MenuItem<BattleMenuOutput>> {
                    match action {
                        CharacterAction::Attack { name, base_damage } => Rc::new(CardMenu {
                            card_name: name.clone(),
                            base_attack: Attack::new(*base_damage),
                            targets: self.targets.clone(),
                        }),
                    }
                })
                .collect(),
        )
    }
}

pub struct CardMenu {
    pub card_name: String,
    pub base_attack: Attack,
    pub targets: Vec<String>,
}

impl MenuItem<BattleMenuOutput> for CardMenu {
    fn label(&self) -> &str {
        &self.card_name
    }

    fn action(&self) -> BattleMenuAction {
        MenuAction::MenuItem(
            self.targets
                .iter()
                .map(|target| -> Rc<dyn MenuItem<BattleMenuOutput>> {
                    Rc::new(CardSelectionItem {
                        target: target.clone(),
                        card_name: self.card_name.clone(),
                        base_attack: self.base_attack,
                    })
                })
                .collect(),
        )
    }
}

pub struct PassMenuItem {}

impl MenuItem<BattleMenuOutput> for PassMenuItem {
    fn label(&self) -> &str {
        "Pass"
    }

    fn action(&self) -> BattleMenuAction {
        MenuAction::Done(BattleMenuOutput::Pass)
    }
}
