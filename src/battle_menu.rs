use std::rc::Rc;

use crate::*;

pub enum BattleMenuOutput {
    Pass,
    Attack {
        target: String,
        attack_name: String,
        base_attack: Attack,
    },
}

type BattleMenuAction = MenuAction<BattleMenuOutput>;

pub type BattleMenu = Menu<BattleMenuOutput>;

pub struct AttackSelectionItem {
    target: String,
    attack_name: String,
    base_attack: Attack,
}

impl MenuItem<BattleMenuOutput> for AttackSelectionItem {
    fn label(&self) -> &str {
        &self.target
    }

    fn action(&self) -> BattleMenuAction {
        MenuAction::Done(BattleMenuOutput::Attack {
            target: self.target.clone(),
            attack_name: self.attack_name.clone(),
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
        "Actions"
    }

    fn action(&self) -> BattleMenuAction {
        BattleMenuAction::MenuItem(
            self.actions
                .iter()
                .map(|action| -> Rc<dyn MenuItem<BattleMenuOutput>> {
                    match action {
                        CharacterAction::Attack { name, base_damage } => Rc::new(AttackMenu {
                            attack_name: name.clone(),
                            base_attack: Attack::new(*base_damage),
                            targets: self.targets.clone(),
                        }),
                    }
                })
                .collect(),
        )
    }
}

pub struct AttackMenu {
    pub attack_name: String,
    pub base_attack: Attack,
    pub targets: Vec<String>,
}

impl MenuItem<BattleMenuOutput> for AttackMenu {
    fn label(&self) -> &str {
        &self.attack_name
    }

    fn action(&self) -> BattleMenuAction {
        MenuAction::MenuItem(
            self.targets
                .iter()
                .map(|target| -> Rc<dyn MenuItem<BattleMenuOutput>> {
                    Rc::new(AttackSelectionItem {
                        target: target.clone(),
                        attack_name: self.attack_name.clone(),
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
