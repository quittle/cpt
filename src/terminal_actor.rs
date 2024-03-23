use std::rc::Rc;

use crate::*;
use async_trait::async_trait;

pub struct TerminalActor {
    pub character: Character,
}

impl TerminalActor {
    fn get_valid_target(
        &self,
        blocks: &mut Vec<TerminalBlock>,
        mut menu: BattleMenu,
        battle: &Battle,
    ) -> Result<CharacterId, ActionError> {
        blocks.push(TerminalBlock::default());
        blocks.push(TerminalBlock {
            prefix: TerminalSpan {
                contents: "".into(),
                color: Some(Box::new(termion::color::Yellow)),
            },
            contents: "".into(),
            ..Default::default()
        });

        let action = menu.wait_for_selection(blocks)?;
        match action {
            BattleMenuOutput::Pass => Err(ActionError::fail("Passing")),
            BattleMenuOutput::Attack(name) => {
                for (_team_id, actor) in &battle.actors {
                    if actor.get_character().name == name {
                        return Ok(actor.get_character().id);
                    }
                }
                Err(ActionError::fail(format!(
                    "Failed to find character id for {name}"
                )))
            }
        }
    }
}

#[async_trait]
impl Actor for TerminalActor {
    async fn act(&self, battle: &Battle) -> ActionResult {
        let mut blocks = vec![];

        let mut enemies = vec![];
        for team in &battle.teams {
            blocks.push(TerminalBlock::new(format!("Team: {}", team.name)));

            for (team_id, actor) in &battle.actors {
                if team_id != &team.id {
                    continue;
                }
                enemies.push(actor.get_character().name.clone());
                blocks.push(TerminalBlock::new(format!(
                    "- {} ({}). Health: {}",
                    actor.get_character().name,
                    actor.get_character().id,
                    actor.get_character().health,
                )));
            }
        }

        blocks.push(TerminalBlock::default());

        let menu = BattleMenu::new(vec![
            Rc::new(AttackMenu { targets: enemies }),
            Rc::new(PassMenuItem {}),
        ]);

        let target: CharacterId = self.get_valid_target(&mut blocks, menu, battle)?;
        Ok(ActionRequest {
            description: target.to_string(),
            action: Action::AttackCharacter(target, self.character.base_attack),
        })
    }

    fn get_character(&self) -> &Character {
        &self.character
    }

    fn get_mut_character(&mut self) -> &mut Character {
        &mut self.character
    }
}
