use std::collections::HashMap;
use std::process::ExitCode;

use futures::future::join_all;
use web_actor::WebActor;

use crate::*;

DeclareWrappedType!(TeamId, id, u64);

#[derive(Debug)]
pub struct Team {
    pub id: TeamId,
    pub name: String,
}

#[derive(Debug)]
struct Turn {
    character: CharacterId,
}

pub struct Battle {
    pub actors: Vec<(TeamId, Box<dyn Actor>)>,
    pub characters: HashMap<CharacterId, Character>,
    pub teams: Vec<Team>,
    pub history: Vec<BattleText>,
    pub random_provider: Box<dyn RandomProvider>,
    pub round: u16,
    pub cards: HashMap<CardId, Card>,
}

unsafe impl Sync for Battle {}

impl Battle {
    pub async fn deserialize(
        data: &str,
        random_provider: Box<dyn RandomProvider>,
    ) -> Result<Self, String> {
        let battle = battle_file::Battle::parse_from_str(data)?;
        let max_team_size = battle
            .teams
            .iter()
            .map(|team| team.members.len())
            .max()
            .unwrap_or(0);
        Ok(Battle {
            history: vec![],
            random_provider,
            characters: battle
                .teams
                .iter()
                .flat_map(|team| &team.members)
                .enumerate()
                .map(|(index, member)| {
                    (
                        CharacterId::new(index),
                        Character {
                            id: CharacterId::new(index),
                            name: member.name.clone(),
                            race: match member.race {
                                battle_file::Race::Human => CharacterRace::Human,
                            },
                            hand: vec![],
                            deck: member
                                .cards
                                .iter()
                                .map(|card_id| CardId::new(*card_id))
                                .collect(),
                            health: Health::new(member.base_health),
                            hand_size: member.hand_size.unwrap_or(battle.default_hand_size),
                        },
                    )
                })
                .collect(),
            cards: battle
                .cards
                .iter()
                .map(|card| {
                    let map_target = |target: &battle_file::Target| match target {
                        battle_file::Target::Me => Target::Me,
                        battle_file::Target::Others => Target::Others,
                    };
                    (
                        CardId::new(card.id),
                        Card {
                            id: CardId::new(card.id),
                            name: card.name.clone(),
                            description: card.description.clone(),
                            flavor: card.flavor.clone(),
                            actions: card
                                .actions
                                .iter()
                                .map(|action| match action {
                                    battle_file::CardAction::Damage { target, amount } => {
                                        CardAction::Damage {
                                            target: map_target(target),
                                            amount: *amount,
                                        }
                                    }
                                    battle_file::CardAction::Heal { target, amount } => {
                                        CardAction::Heal {
                                            target: map_target(target),
                                            amount: *amount,
                                        }
                                    }
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            teams: battle
                .teams
                .iter()
                .enumerate()
                .map(|(index, team)| Team {
                    id: TeamId::new(index.try_into().unwrap()),
                    name: team.name.clone(),
                })
                .collect(),
            actors: join_all(
                battle
                    .teams
                    .iter()
                    .enumerate()
                    .flat_map(|(team_index, team)| {
                        team.members
                            .iter()
                            .enumerate()
                            .map(move |(member_index, team_member)| {
                                let character_id =
                                    CharacterId::new(team_index * max_team_size + member_index);
                                async move {
                                    (
                                        TeamId::new(team_index.try_into().unwrap()),
                                        if team_member.is_player {
                                            if cfg!(feature = "terminal_ui") {
                                                Box::new(TerminalActor { character_id })
                                                    as Box<dyn Actor>
                                            } else {
                                                Box::new(WebActor::new(character_id).await.unwrap())
                                                    as Box<dyn Actor>
                                            }
                                        } else {
                                            Box::new(DumbActor { character_id }) as Box<dyn Actor>
                                        },
                                    )
                                }
                            })
                    }),
            )
            .await,
            round: 0,
        })
    }

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

    fn handle_action(&mut self, action: Action, character_id: &CharacterId) {
        let character = &self.characters[character_id];
        match action {
            Action::Pass => {
                self.history.push(battle_markup![
                    @id(&character.name),
                    " took no action",
                ]);
            }
            Action::Act(card_id, target_id) => {
                let card = &self.cards[&card_id];
                let target_character = &self.characters[&target_id];
                let mut history_entry = battle_markup![
                    @id(&character.name),
                    " used ",
                    @attack(&card.name),
                    " on ",
                    @id(&target_character.name),
                    ". "
                ];

                for action in &card.actions {
                    match action {
                        CardAction::Damage { amount, .. } => {
                            history_entry.extend(battle_markup![@damage(amount), " damage"]);

                            self.characters.get_mut(&target_id).unwrap().health -=
                                Attack::new(*amount);
                        }
                        CardAction::Heal { amount, .. } => {
                            history_entry.extend(battle_markup!["Healed ", @damage(amount)]);

                            self.characters.get_mut(&target_id).unwrap().health +=
                                Health::new(*amount);
                        }
                    }
                }

                self.history.push(history_entry);
            }
        }
    }

    pub async fn advance(&mut self) -> Result<(), ExitCode> {
        self.round += 1;
        self.history
            .push(battle_markup![format!("--- Round {}", self.round)]);
        let turns = self.build_turns();
        for turn in turns {
            self.characters
                .get_mut(&turn.character)
                .unwrap()
                .reset_hand(self.random_provider.as_ref());

            let character = &self.characters[&turn.character];
            if character.is_dead() {
                continue;
            }
            let actor: &dyn Actor = self.require_actor(&turn.character);
            let action_result = actor.act(self).await;
            match action_result {
                Ok(request) => self.handle_action(request, &turn.character),
                Err(ActionError::Failure(failure)) => {
                    println!("Error processing {}: {}", turn.character, failure.message);
                }
                Err(ActionError::Exit(exit_code)) => {
                    return Err(exit_code);
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
                            "hand_size": 1
                        },
                        {
                            "name": "Member A2",
                            "race": "Human",
                            "base_health": 5,
                            "cards": [1]
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
                            "cards": [0]
                        }
                    ]
                }
            ]
        }"#;
        let mut battle =
            Battle::deserialize(battle_json, Box::<DefaultRandomProvider>::default()).await?;
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
