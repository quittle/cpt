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

pub enum BattleHistory {
    Text(String),
    Id(String),
    Attack(String),
    Damage(String),
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
        let attacks = &battle.attacks;
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
                                    .attacks
                                    .iter()
                                    .map(|attack_id| {
                                        let attack: &battle_file::Attack =
                                            &attacks[*attack_id as usize];
                                        assert_eq!(
                                            attack.id, *attack_id,
                                            "Should have already been verified in battle_file.",
                                        );
                                        CharacterAction::Attack {
                                            name: attack.name.clone(),
                                            base_damage: attack.base_damage,
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

    pub fn has_more_than_one_team_alive(&self) -> bool {
        let mut cur_id = None;
        for (team_id, actor) in &self.actors {
            if !actor.get_character().is_dead() {
                if cur_id.is_some() && cur_id != Some(team_id) {
                    return true;
                }
                cur_id = Some(team_id);
            }
        }
        false
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
                            BattleHistory::Id(actor.get_character().name.clone()),
                            BattleHistory::Text("used".into()),
                            BattleHistory::Attack(attack_name),
                            BattleHistory::Text("on".into()),
                            BattleHistory::Id(
                                self.require_actor(target).get_character().name.clone(),
                            ),
                            BattleHistory::Text("for".into()),
                            BattleHistory::Damage(attack.to_string()),
                            BattleHistory::Text("damage".into()),
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
            if !self.has_more_than_one_team_alive() {
                return;
            }
        }
    }

    pub async fn run_to_completion(&mut self) {
        while self.has_more_than_one_team_alive() {
            self.advance().await;
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
            "attacks": [
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
                            "attacks": [0]
                        },
                        {
                            "name": "Member A2",
                            "race": "Human",
                            "base_health": 5,
                            "attacks": [1]
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
                            "attacks": [0]
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
