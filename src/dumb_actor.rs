use std::cmp::Ordering;

use crate::*;
use async_trait::async_trait;

pub struct DumbActor {
    pub character_id: CharacterId,
}

fn _pick_random_card(character: &Character, battle: &Battle) -> Option<CardId> {
    character
        .hand
        .pick_linear(battle.random_provider.as_ref())
        .copied()
}

fn total_average_damage(card: &Card) -> u64 {
    card.actions
        .iter()
        .map(|action| match action {
            CardAction::Damage {
                target: _,
                amount,
                area: _, // TODO: Evaluate area damage
            } => (amount.0 + amount.1) / 2,
            _ => 0,
        })
        .sum()
}

fn prioritize_cards(character: &Character, battle: &Battle) -> Vec<CardId> {
    let mut sorted = character.hand.clone();
    sorted.sort_unstable_by(|a, b| {
        let card_a = &battle.cards[a];
        let card_b = &battle.cards[b];
        let damage_a = total_average_damage(card_a);
        let damage_b = total_average_damage(card_b);

        match damage_a.cmp(&damage_b) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            _ => {}
        }

        Ordering::Greater
    });
    sorted
}

#[async_trait]
impl Actor for DumbActor {
    fn get_character_id(&self) -> &CharacterId {
        &self.character_id
    }

    async fn act(&self, battle: &Battle) -> ActionResult {
        let my_team = battle
            .get_team_for_actor(self)
            .unwrap_or_else(|| panic!("Failed to find team for self {}", self.character_id));
        let character = battle.get_character(self);

        let prioritized_cards = prioritize_cards(character, battle);
        for card_id in prioritized_cards {
            let card = &battle.cards[&card_id];
            if (card.target() == Target::Me
                && total_average_damage(card) < battle.characters[&self.character_id].health.health)
                || (card.target() == Target::Any && total_average_damage(card) == 0)
            {
                return Ok(Action::Act(card_id, self.character_id));
            }

            for (team_id, actor) in &battle.actors {
                let opponent = &battle.characters[actor.get_character_id()];
                if &my_team != team_id && !opponent.is_dead() {
                    if let Some(distance) = battle.board.distance(
                        BoardItem::Character(character.id),
                        BoardItem::Character(opponent.id),
                    ) {
                        if card.range >= distance && character.remaining_actions > 0 {
                            return Ok(Action::Act(card_id, opponent.id));
                        } else if character.movement > 0 {
                            if let Some(path) = battle.board.shortest_path(
                                BoardItem::Character(character.id),
                                BoardItem::Character(opponent.id),
                            ) {
                                // Only try moving if there's more than 2 spots (current location and target location)
                                if path.len() > 2 {
                                    return Ok(Action::Move(character.id, path[1].clone()));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(Action::Pass)
    }

    async fn on_game_over(&self, _battle: &Battle) {}
}
