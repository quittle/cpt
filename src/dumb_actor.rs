use crate::*;
use async_trait::async_trait;

pub struct DumbActor {
    pub character: Character,
}

#[async_trait]
impl Actor for DumbActor {
    async fn act(&self, battle: &Battle) -> ActionResult {
        let my_team = battle.get_team_for_actor(self).expect(&format!(
            "Failed to find team for self {}",
            self.character.id
        ));
        for (team_id, actor) in &battle.actors {
            if &my_team != team_id {
                return Ok(ActionRequest {
                    description: "Attack".into(),
                    action: Action::AttackCharacter(
                        actor.get_character().id,
                        self.character.base_attack,
                    ),
                });
            }
        }

        Ok(ActionRequest {
            description: "Doing Nothing".into(),
            action: Action::Pass,
        })
    }

    fn get_character(&self) -> &Character {
        &self.character
    }

    fn get_mut_character(&mut self) -> &mut Character {
        &mut self.character
    }
}
