use crate::*;
use async_trait::async_trait;
use std::io::{self, Write};

pub struct TerminalActor {
    pub character: Character,
}

impl TerminalActor {
    fn get_valid_target(&self, battle: &Battle) -> Result<CharacterId, ActionFailure> {
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
        let mut target: Option<CharacterId> = None;
        while target.is_none() || !valid_targets.contains(&target.unwrap_or(CharacterId::INVALID)) {
            let mut line = String::new();
            print!(
                "Who should {}({}) attack? ",
                self.character.name, self.character.id
            );
            io::stdout().flush()?;
            io::stdin().read_line(&mut line)?;
            target = CharacterId::parse(line.trim());
        }
        Ok(target.unwrap())
    }
}

#[async_trait]
impl Actor for TerminalActor {
    async fn act(&self, battle: &Battle, _character_id: CharacterId) -> ActionResult {
        println!();

        for team in &battle.teams {
            println!("Team: {}", team.name);
            for (team_id, actor) in &battle.actors {
                if team_id != &team.id {
                    continue;
                }
                println!(
                    "- {} ({}). Health: {}",
                    actor.get_character().name,
                    actor.get_character().id,
                    actor.get_character().health,
                );
            }
        }

        let target: CharacterId = self.get_valid_target(battle)?;
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
