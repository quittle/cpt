use crate::*;
use async_trait::async_trait;

pub struct DumbActor {
    pub character_id: CharacterId,
}

fn pick_random_card(character: &Character, battle: &Battle) -> Option<CardId> {
    character
        .hand
        .pick_linear(battle.random_provider.as_ref())
        .copied()
}

#[async_trait]
impl Actor for DumbActor {
    fn get_character_id(&self) -> &CharacterId {
        &self.character_id
    }

    async fn act(&self, battle: &Battle) -> ActionResult {
        let should_attack = random_choice!(battle.random_provider, true, false);
        if !should_attack {
            return Ok(Action::Pass);
        }

        let my_team = battle
            .get_team_for_actor(self)
            .unwrap_or_else(|| panic!("Failed to find team for self {}", self.character_id));
        let character = battle.get_character(self);
        for (team_id, actor) in &battle.actors {
            if &my_team != team_id && !character.is_dead() {
                if let Some(card_id) = pick_random_card(character, battle) {
                    return Ok(Action::Act(card_id, *actor.get_character_id()));
                }
            }
        }

        Ok(Action::Pass)
    }

    async fn on_game_over(&self, _battle: &Battle) {}
}
