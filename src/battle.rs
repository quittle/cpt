use crate::{
    battle_file, battle_markup, Action, ActionError, Actor, Attack, BattleText, Board, Card,
    CardAction, CardId, Character, CharacterId, DeclareWrappedType, Health, RandomProvider, Target,
};
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::ExitCode;

DeclareWrappedType!(TeamId, id, u64);

#[derive(Debug, Serialize)]
pub struct Team {
    pub id: TeamId,
    pub name: String,
}

#[derive(Debug)]
struct Turn {
    character: CharacterId,
}

type StoryCard = battle_file::StoryCard;

#[derive(Serialize)]
pub struct Battle {
    #[serde(skip)]
    pub actors: Vec<(TeamId, Box<dyn Actor>)>,
    pub characters: HashMap<CharacterId, Character>,
    pub introduction: Option<StoryCard>,
    pub teams: Vec<Team>,
    pub history: Vec<BattleText>,
    #[serde(skip)]
    pub random_provider: Box<dyn RandomProvider>,
    pub round: u16,
    pub cards: HashMap<CardId, Card>,
    pub default_turn_actions: u8,
    #[serde(skip)]
    pub asset_directory: Option<PathBuf>,
    pub board: Board,
}

unsafe impl Sync for Battle {}

impl Battle {
    pub fn get_character(&self, actor: &dyn Actor) -> &Character {
        &self.characters[actor.get_character_id()]
    }

    pub fn get_team_for_actor(&self, actor: &dyn Actor) -> Option<TeamId> {
        for (team_id, other_actor) in &self.actors {
            if actor.get_character_id() == other_actor.get_character_id() {
                return Some(*team_id);
            }
        }
        None
    }

    pub fn get_team_from_id(&self, id: TeamId) -> Option<&Team> {
        self.teams.iter().find(|&team| team.id == id)
    }

    fn build_turns(&self) -> Vec<Turn> {
        let mut ret = vec![];
        for (_team_id, actor) in &self.actors {
            if self.get_character(actor.as_ref()).is_dead() {
                continue;
            }

            ret.push(Turn {
                character: *actor.get_character_id(),
            });
        }
        ret
    }

    pub fn get_actor(&self, character_id: &CharacterId) -> Option<&dyn Actor> {
        for (_team_id, actor) in &self.actors {
            if actor.get_character_id() == character_id {
                return Some(actor.as_ref());
            }
        }
        None
    }

    pub fn get_mut_actor(&mut self, character_id: &CharacterId) -> Option<&mut dyn Actor> {
        for (_team_id, actor) in &mut self.actors {
            if actor.get_character_id() == character_id {
                return Some(actor.as_mut());
            }
        }
        None
    }

    pub fn require_actor(&self, character_id: &CharacterId) -> &dyn Actor {
        self.get_actor(character_id)
            .unwrap_or_else(|| panic!("Unable to find actor with character id: {character_id}"))
    }

    pub fn require_mut_actor(&mut self, character_id: &CharacterId) -> &mut dyn Actor {
        self.get_mut_actor(character_id)
            .unwrap_or_else(|| panic!("Unable to find actor with character id: {character_id}"))
    }

    /// Checks if only one team is alive and returns that team. Returns None if multiple teams are alive or if None are
    pub fn check_only_one_team_alive(&self) -> Option<TeamId> {
        let mut cur_id = None;
        for (team_id, actor) in &self.actors {
            if !self.get_character(actor.as_ref()).is_dead() {
                if cur_id.is_some() && cur_id != Some(*team_id) {
                    return None;
                }
                cur_id = Some(*team_id);
            }
        }
        cur_id
    }

    /// Attempts to carry out the action. If illegal, returns false
    fn handle_action(&mut self, actor: &CharacterId, action: Action) -> bool {
        let character = &self.characters[actor];
        match action {
            Action::Pass => {
                self.history.push(battle_markup![
                    @id(&character.name),
                    " took no action",
                ]);
            }
            Action::Act(card_id, target_id) => {
                let card = &self.cards[&card_id];
                let actual_target = if *actor == target_id {
                    Target::Me
                } else {
                    Target::Others
                };

                if !card.target().is_super_set(&actual_target) {
                    return false;
                }

                let target_character = &self.characters[&target_id];
                if target_character.is_dead() {
                    return false;
                }

                let mut history_entry = battle_markup![
                    @id(&character.name),
                    " used ",
                    @attack(&card.name),
                    " on ",
                    @id(&target_character.name),
                    ". "
                ];

                for action in &card.actions {
                    // If the action specifically targets me, then force it to target the actor
                    // rather than the potentially other target.
                    let target_id = if action.target() == &Target::Me {
                        actor
                    } else {
                        &target_id
                    };

                    let target_character = self.characters.get_mut(target_id).unwrap();
                    match action {
                        CardAction::Damage { amount, .. } => {
                            let value = amount.resolve(self.random_provider.as_ref());
                            history_entry.extend(battle_markup![@damage(&value), " damage. "]);

                            target_character.health -= Attack::new(value);
                        }
                        CardAction::Heal { amount, .. } => {
                            let value = amount.resolve(self.random_provider.as_ref());
                            history_entry.extend(battle_markup!["Healed ", @damage(&value), ". "]);

                            target_character.heal(Health::new(value));
                        }
                        CardAction::GainAction { amount, .. } => {
                            history_entry
                                .extend(battle_markup![format!("Gained {} action. ", amount)]);
                            target_character.remaining_actions += *amount;
                        }
                    }
                }

                self.history.push(history_entry);

                // Remove card from hand
                let hand = &mut self.characters.get_mut(actor).unwrap().hand;
                hand.remove(hand.iter().position(|id| id == &card_id).unwrap());
            }
        }
        true
    }

    pub async fn advance(&mut self) -> Result<(), ExitCode> {
        self.round += 1;
        self.history
            .push(battle_markup![format!("--- Round {}", self.round)]);
        let turns = self.build_turns();
        for turn in turns {
            let character = self.characters.get_mut(&turn.character).unwrap();
            character.reset_hand(self.random_provider.as_ref());
            character.remaining_actions = character
                .get_default_turn_actions()
                .unwrap_or(self.default_turn_actions);

            if character.is_dead() {
                continue;
            }

            while self.characters[&turn.character].remaining_actions > 0 {
                let actor: &dyn Actor = self.require_actor(&turn.character);
                let action_result = actor.act(self).await;
                match action_result {
                    Ok(request) => {
                        if self.handle_action(&turn.character, request) {
                            self.characters
                                .get_mut(&turn.character)
                                .unwrap()
                                .remaining_actions -= 1;
                        }
                    }
                    Err(ActionError::Failure(failure)) => {
                        println!("Error processing {}: {}", turn.character, failure.message);
                    }
                    Err(ActionError::Exit(exit_code)) => {
                        return Err(exit_code);
                    }
                }
            }
            if self.check_only_one_team_alive().is_some() {
                return Ok(());
            }
        }
        Ok(())
    }

    pub async fn run_to_completion(&mut self) -> Result<(), ExitCode> {
        let mut surviving_team = None;
        while surviving_team.is_none() {
            self.advance().await?;
            surviving_team = self.check_only_one_team_alive()
        }
        let team_id = surviving_team.unwrap();
        let team = self.get_team_from_id(team_id).unwrap();
        self.history
            .push(battle_markup![format!("{} won.", team.name)]);

        for (_, actor) in &self.actors {
            actor.on_game_over(self).await;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::{Battle, DefaultRandomProvider};

    #[tokio::test]
    async fn test_deserialize() -> Result<(), String> {
        let battle_json = r#"{
            "title": "Example Game",
            "description": "Example Description",
            "default_hand_size": 2,
            "board": { "width": 2, "height": 2 },
            "cards": [
                {
                    "id": 0,
                    "name": "Kick",
                    "description": "Deal 123 damage",
                    "actions": [
                        {
                            "type": "damage",
                            "target": "others",
                            "amount": 123
                        }
                    ]
                },
                {
                    "id": 1,
                    "name": "Punch",
                    "description": "Deal 456 damage",
                    "actions": [
                        {
                            "type": "damage",
                            "target": "others",
                            "amount": 456
                        }
                    ]
                }
            ],
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Member A1",
                            "race": "Human",
                            "base_health": 5,
                            "cards": [0],
                            "hand_size": 1,
                            "location": [0, 0]
                        },
                        {
                            "name": "Member A2",
                            "race": "Human",
                            "base_health": 5,
                            "cards": [1],
                            "location": [0, 1]
                        }
                    ]
                },
                {
                    "name": "Team B",
                    "members": [
                        {
                            "name": "Member B1",
                            "race": "Human",
                            "base_health": 15,
                            "cards": [0],
                            "location": [1, 0]
                        }
                    ]
                }
            ]
        }"#;
        let mut battle =
            Battle::deserialize(battle_json, None, Box::<DefaultRandomProvider>::default()).await?;
        assert_eq!(battle.history.len(), 0);
        assert_eq!(battle.teams.len(), 2);
        assert_eq!(battle.teams[0].name, "Team A".to_string());
        assert_eq!(battle.teams[0].id.id, 0);
        assert_eq!(battle.teams[1].name, "Team B".to_string());
        assert_eq!(battle.teams[1].id.id, 1);
        assert_eq!(battle.actors.len(), 3);
        assert_eq!(battle.actors[0].0.id, 0);
        assert_eq!(
            battle.characters[battle.actors[0].1.get_character_id()].name,
            "Member A1"
        );
        assert_eq!(
            battle.characters[battle.actors[0].1.get_character_id()].hand_size,
            1
        );
        assert_eq!(battle.actors[1].0.id, 0);
        assert_eq!(
            battle.characters[battle.actors[1].1.get_character_id()].name,
            "Member A2"
        );
        assert_eq!(
            battle.characters[battle.actors[1].1.get_character_id()].hand_size,
            2
        );
        assert_eq!(battle.actors[2].0.id, 1);
        assert_eq!(
            battle.characters[battle.actors[2].1.get_character_id()].name,
            "Member B1"
        );

        block_on(battle.run_to_completion()).unwrap();
        Ok(())
    }
}
