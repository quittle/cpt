use crate::*;
use async_trait::async_trait;

pub struct DumbActor {
    pub character: Character,
}

fn pick_random_card(character: &Character, battle: &Battle) -> CardId {
    *character
        .cards
        .pick_linear(battle.random_provider.as_ref())
        .unwrap()
}

#[async_trait]
impl Actor for DumbActor {
    async fn act(&self, battle: &Battle) -> ActionResult {
        let should_attack = random_choice!(battle.random_provider, true, false);
        if !should_attack {
            return Ok(Action::Pass);
        }

        let my_team = battle
            .get_team_for_actor(self)
            .unwrap_or_else(|| panic!("Failed to find team for self {}", self.character.id));
        for (team_id, actor) in &battle.actors {
            if &my_team != team_id && !actor.get_character().is_dead() {
                let card_id = pick_random_card(&self.character, battle);
                return Ok(Action::Act(card_id, actor.get_character().id));
            }
        }

        Ok(Action::Pass)
    }

    fn get_character(&self) -> &Character {
        &self.character
    }

    fn get_mut_character(&mut self) -> &mut Character {
        &mut self.character
    }

    async fn on_game_over(&self, _battle: &Battle) {}
}
