use std::rc::Rc;

use crate::{Menu, MenuAction, MenuItem};

pub enum BattleMenuOutput {
    Pass,
    Attack(String),
}

type BattleMenuAction = MenuAction<BattleMenuOutput>;

pub type BattleMenu = Menu<BattleMenuOutput>;

pub struct AttackSelectionItem {
    target: String,
}

impl MenuItem<BattleMenuOutput> for AttackSelectionItem {
    fn label(&self) -> &str {
        &self.target
    }

    fn action(&self) -> BattleMenuAction {
        println!("Attacking {}", self.target);
        MenuAction::Done(BattleMenuOutput::Attack(self.target.clone()))
    }
}

pub struct AttackMenu {
    pub targets: Vec<String>,
}

impl MenuItem<BattleMenuOutput> for AttackMenu {
    fn label(&self) -> &str {
        "Attack"
    }

    fn action(&self) -> BattleMenuAction {
        MenuAction::MenuItem(
            self.targets
                .iter()
                .map(|target| -> Rc<dyn MenuItem<BattleMenuOutput>> {
                    Rc::new(AttackSelectionItem {
                        target: target.clone(),
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
