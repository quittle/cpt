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
    pub teams: Vec<Team>,
    pub history: Vec<Vec<BattleHistory>>,
    pub random_provider: Box<dyn RandomProvider>,
    pub round: u16,
}

unsafe impl Sync for Battle {}

impl Battle {
    pub fn deserialize(
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
        let cards = &battle.cards;
        Ok(Battle {
            history: vec![],
            random_provider,
            teams: battle
                .teams
                .iter()
                .enumerate()
                .map(|(index, team)| Team {
                    id: TeamId::new(index.try_into().unwrap()),
                    name: team.name.clone(),
                })
                .collect(),
            actors: battle
                .teams
                .iter()
                .enumerate()
                .flat_map(|(team_index, team)| {
                    team.members
                        .iter()
                        .enumerate()
                        .map(move |(member_index, team_member)| {
                            let character = Character {
                                id: CharacterId::new(
                                    (team_index * max_team_size + member_index)
                                        .try_into()
                                        .unwrap(),
                                ),
                                name: team_member.name.clone(),
                                race: match team_member.race {
                                    battle_file::Race::Human => CharacterRace::Human,
                                },
                                actions: team_member
                                    .cards
                                    .iter()
                                    .map(|card_id| {
                                        let card: &battle_file::Card = &cards[*card_id as usize];
                                        assert_eq!(
                                            card.id, *card_id,
                                            "Should have already been verified in battle_file.",
                                        );
                                        CharacterAction::Attack {
                                            name: card.name.clone(),
                                            base_damage: card.base_damage,
                                        }
                                    })
                                    .collect(),
                                health: Health::new(team_member.base_health),
                            };

                            (
                                TeamId::new(team_index.try_into().unwrap()),
                                if team_member.is_player {
                                    Box::new(TerminalActor { character }) as Box<dyn Actor>
                                } else {
                                    Box::new(DumbActor { character }) as Box<dyn Actor>
                                },
                            )
                        })
                })
                .collect(),
            round: 0,
        })
    }

    pub fn get_team_for_actor(&self, actor: &dyn Actor) -> Option<TeamId> {
        for (team_id, other_actor) in &self.actors {
            if actor.get_character().id == other_actor.get_character().id {
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
            if actor.get_character().is_dead() {
                continue;
            }

            ret.push(Turn {
                character: actor.get_character().id,
            });
        }
        ret
    }

    fn get_actor(&self, character_id: CharacterId) -> Option<&dyn Actor> {
        for (_team_id, actor) in &self.actors {
            if actor.get_character().id == character_id {
                return Some(actor.as_ref());
            }
        }
        None
    }

    fn require_actor(&self, character_id: CharacterId) -> &dyn Actor {
        self.get_actor(character_id)
            .unwrap_or_else(|| panic!("Unable to find actor with character id: {character_id}"))
    }

    fn get_mut_actor(&mut self, character_id: CharacterId) -> Option<&mut dyn Actor> {
        for (_team_id, actor) in &mut self.actors {
            if actor.get_character().id == character_id {
                return Some(actor.as_mut());
            }
        }
        None
    }

    fn require_mut_actor(&mut self, character_id: CharacterId) -> &mut dyn Actor {
        self.get_mut_actor(character_id)
            .unwrap_or_else(|| panic!("Unable to find actor with character id: {character_id}"))
    }

    /// Checks if only one team is alive and returns that team. Returns None if multiple teams are alive or if None are
    pub fn check_only_one_team_alive(&self) -> Option<TeamId> {
        let mut cur_id = None;
        for (team_id, actor) in &self.actors {
            if !actor.get_character().is_dead() {
                if cur_id.is_some() && cur_id != Some(*team_id) {
                    return None;
                }
                cur_id = Some(*team_id);
            }
        }
        cur_id
    }

    pub async fn advance(&mut self) {
        self.round += 1;
        self.history.push(vec![BattleHistory::Text(format!(
            "--- Round {}",
            self.round
        ))]);
        let turns = self.build_turns();
        for turn in turns {
            let actor = self.require_actor(turn.character);
            if actor.get_character().is_dead() {
                continue;
            }
            let action_result = actor.act(self).await;
            match action_result {
                Ok(request) => match request.action {
                    Action::Pass => {
                        self.history.push(vec![
                            BattleHistory::Id(actor.get_character().name.clone()),
                            BattleHistory::Text("took no action".into()),
                        ]);
                    }
                    Action::AttackCharacter(target, attack_name, attack) => {
                        self.history.push(vec![
                            BattleHistory::id(&actor.get_character().name),
                            BattleHistory::text("used"),
                            BattleHistory::attack(attack_name),
                            BattleHistory::text("on"),
                            BattleHistory::id(&self.require_actor(target).get_character().name),
                            BattleHistory::text("for"),
                            BattleHistory::damage(attack),
                            BattleHistory::text("damage"),
                        ]);

                        let target_actor = self.require_mut_actor(target);
                        target_actor.damage(turn.character, attack);
                    }
                },
                Err(ActionError::Failure(failure)) => {
                    println!("Error processing {}: {}", turn.character, failure.message);
                }
                Err(ActionError::Exit(exit_code)) => {
                    std::process::exit(exit_code);
                }
            }
            if self.check_only_one_team_alive().is_some() {
                return;
            }
        }
    }

    pub async fn run_to_completion(&mut self) {
        let mut surviving_team = None;
        while surviving_team.is_none() {
            self.advance().await;
            surviving_team = self.check_only_one_team_alive()
        }
        let team_id = surviving_team.unwrap();
        let team = self.get_team_from_id(team_id).unwrap();
        self.history
            .push(vec![BattleHistory::text(format!("{} won.", team.name))]);

        for (_, actor) in &self.actors {
            actor.on_game_over(self).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::{Battle, DefaultRandomProvider};

    #[test]
    fn test_deserialize() -> Result<(), String> {
        let battle_json = r#"{
            "title": "Example Game",
            "description": "Example Description",
            "cards": [
                {
                    "id": 0,
                    "name": "Kick",
                    "base_damage": 123
                },
                {
                    "id": 1,
                    "name": "Punch",
                    "base_damage": 456
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
                            "cards": [0]
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
        let mut battle = Battle::deserialize(battle_json, Box::<DefaultRandomProvider>::default())?;
        assert_eq!(battle.history.len(), 0);
        assert_eq!(battle.teams.len(), 2);
        assert_eq!(battle.teams[0].name, "Team A".to_string());
        assert_eq!(battle.teams[0].id.id, 0);
        assert_eq!(battle.teams[1].name, "Team B".to_string());
        assert_eq!(battle.teams[1].id.id, 1);
        assert_eq!(battle.actors.len(), 3);
        assert_eq!(battle.actors[0].0.id, 0);
        assert_eq!(battle.actors[0].1.get_character().name, "Member A1");
        assert_eq!(battle.actors[1].0.id, 0);
        assert_eq!(battle.actors[1].1.get_character().name, "Member A2");
        assert_eq!(battle.actors[2].0.id, 1);
        assert_eq!(battle.actors[2].1.get_character().name, "Member B1");

        block_on(battle.run_to_completion());
        Ok(())
    }
}
