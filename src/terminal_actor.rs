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
        blocks.push(TerminalBlock {
            prefix: TerminalSpan {
                contents: "".into(),
                color: Some(Box::new(termion::color::Yellow)),
            },
            contents: format!(
                "Who should {}({}) attack? ",
                self.character.name, self.character.id
            ),
            ..Default::default()
        });
        loop {
            TerminalUi::draw(blocks)?;

            let (_raw_out, _raw_err) = (
                std::io::stdout().into_raw_mode(),
                std::io::stderr().into_raw_mode(),
            );

            fn last_block(blocks: &mut [TerminalBlock]) -> &mut TerminalBlock {
                blocks.last_mut().unwrap()
            }

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
            if let Some(target) = CharacterId::parse(last_block(blocks).suffix.contents.trim()) {
                if valid_targets.contains(&target) {
                    return Ok(target);
                } else {
                    last_block(blocks).prefix.contents = "Invalid character id ".into();
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
