use std::rc::Rc;

use crate::*;
use async_trait::async_trait;
use termion::{
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
};

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
        loop {
            fn last_block(blocks: &mut [TerminalBlock]) -> &mut TerminalBlock {
                blocks.last_mut().unwrap()
            }

            menu.show(last_block(blocks));
            TerminalUi::draw(blocks)?;

            let (_raw_out, _raw_err) = (
                std::io::stdout().into_raw_mode()?,
                std::io::stderr().into_raw_mode()?,
            );

            for c in std::io::stdin().events() {
                let evt = c.unwrap();
                match evt {
                    Event::Key(Key::Char(c)) => {
                        if c == '\n' {
                            break;
                        }
                        last_block(blocks).suffix.contents.push(c);
                    }
                    Event::Key(Key::Backspace) => {
                        last_block(blocks).suffix.contents.pop();
                    }
                    Event::Key(Key::Ctrl('c' | 'd')) => {
                        println!("Exiting");
                        return Err(ActionError::Exit(13));
                    }
                    Event::Key(k) => {
                        last_block(blocks).prefix.contents = format!("Key: {:?} ", k);
                    }
                    _ => (),
                };
                TerminalUi::draw(blocks)?;
            }
            if let Some(output) = menu.select_by_name(&last_block(blocks).suffix.contents) {
                last_block(blocks).prefix.contents.clear();
                match output {
                    BattleMenuOutput::Pass => {
                        return Err(ActionError::Failure(ActionFailure { message: "".into() }))
                    }
                    BattleMenuOutput::Attack(name) => {
                        for (_team_id, actor) in &battle.actors {
                            if actor.get_character().name == name {
                                return Ok(actor.get_character().id);
                            }
                        }
                        return Err(ActionError::Failure(ActionFailure {
                            message: "Failed to find character id for {name}".into(),
                        }));
                    }
                }
            } else {
                last_block(blocks).prefix.contents = "Unknown command ".into();
            }
            last_block(blocks).suffix.contents.clear();
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
