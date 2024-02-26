use crate::*;
use async_trait::async_trait;
use std::io::Write;
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
        battle: &Battle,
    ) -> Result<CharacterId, ActionError> {
        let team_id = battle
            .actors
            .iter()
            .find_map(|(team_id, actor)| {
                if actor.get_character().id == self.character.id {
                    Some(team_id)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("Actor {} not found in battle", self.character.id));
        let valid_targets: Vec<CharacterId> = battle
            .actors
            .iter()
            .filter_map(|(actor_team_id, actor)| {
                if team_id == actor_team_id {
                    None
                } else {
                    Some(actor.get_character().id)
                }
            })
            .collect();
        blocks.push(TerminalBlock::new_with_suffix(
            format!(
                "Who should {}({}) attack? ",
                self.character.name, self.character.id
            ),
            String::new(),
        ));
        loop {
            TerminalUi::draw(blocks)?;

            let (_raw_out, _raw_err) = (
                std::io::stdout().into_raw_mode(),
                std::io::stderr().into_raw_mode(),
            );

            fn last_string(blocks: &mut [TerminalBlock]) -> &mut String {
                &mut blocks.last_mut().unwrap().suffix.as_mut().unwrap().contents
            }

            for c in std::io::stdin().events() {
                let evt = c.unwrap();
                match evt {
                    Event::Key(Key::Char(c)) => {
                        if c == '\n' {
                            break;
                        }
                        last_string(blocks).push(c);
                        TerminalUi::draw(blocks)?;
                    }
                    Event::Key(Key::Backspace) => {
                        last_string(blocks).pop();
                        TerminalUi::draw(blocks)?;
                    }
                    Event::Key(Key::Ctrl('c' | 'd')) => {
                        println!("Exiting");
                        return Err(ActionError::Exit(13));
                    }
                    Event::Key(k) => {
                        write!(std::io::stdout(), "Key: {:?}", k)?;
                        std::io::stdout().flush().unwrap();
                    }
                    _ => (),
                };
            }
            if let Some(target) = CharacterId::parse(last_string(blocks).trim()) {
                if valid_targets.contains(&target) {
                    return Ok(target);
                }
            }
            last_string(blocks).clear();
        }
    }
}

#[async_trait]
impl Actor for TerminalActor {
    async fn act(&self, battle: &Battle) -> ActionResult {
        let mut blocks = vec![];

        for team in &battle.teams {
            blocks.push(TerminalBlock::new(format!("Team: {}", team.name)));

            for (team_id, actor) in &battle.actors {
                if team_id != &team.id {
                    continue;
                }
                blocks.push(TerminalBlock::new(format!(
                    "- {} ({}). Health: {}",
                    actor.get_character().name,
                    actor.get_character().id,
                    actor.get_character().health,
                )));
            }
        }

        let target: CharacterId = self.get_valid_target(&mut blocks, battle)?;
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
