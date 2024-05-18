use crate::*;
use async_trait::async_trait;

pub struct DumbActor {
    pub character: Character,
}

fn pick_random_card(character: &Character, battle: &Battle) -> (String, Attack) {
    let action = character
        .actions
        .pick_linear(battle.random_provider.as_ref())
        .unwrap();
    match action {
        CharacterAction::Attack { name, base_damage } => {
            (name.to_string(), Attack::new(*base_damage))
        }
    }
}

#[async_trait]
impl Actor for DumbActor {
    async fn act(&self, battle: &Battle) -> ActionResult {
        let should_attack = random_choice!(battle.random_provider, true, false);
        if !should_attack {
            return Ok(ActionRequest {
                description: "Doing Nothing".into(),
                action: Action::Pass,
            });
        }

        let my_team = battle
            .get_team_for_actor(self)
            .unwrap_or_else(|| panic!("Failed to find team for self {}", self.character.id));
        for (team_id, actor) in &battle.actors {
            if &my_team != team_id && !actor.get_character().is_dead() {
                let (attack_name, attack) = pick_random_card(&self.character, battle);
                return Ok(ActionRequest {
                    description: "Attack".into(),
                    action: Action::AttackCharacter(actor.get_character().id, attack_name, attack),
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

    async fn on_game_over(&self, _battle: &Battle) {}
}
