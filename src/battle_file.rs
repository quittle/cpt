use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Battle {
    pub title: String,
    pub description: String,
    pub teams: Vec<Team>,
}

impl Battle {
    pub fn parse_from_str(data: &str) -> Result<Self, String> {
        let battle: Battle = serde_json::from_str(data).map_err(|err| err.to_string())?;

        let mut player_found = false;
        for team in &battle.teams {
            for team_member in &team.members {
                if team_member.is_player {
                    if player_found {
                        return Err("Multiple playable team members found.")?;
                    }
                    player_found = true;
                }
            }
        }

        Ok(battle)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub name: String,
    pub members: Vec<TeamMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamMember {
    pub name: String,
    pub race: Race,
    pub base_health: i64,
    pub attacks: Vec<Attack>,
    #[serde(default)]
    pub is_player: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Race {
    Human,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attack {
    pub name: String,
    pub base_damage: i64,
}

#[cfg(test)]
mod tests {
    use super::Battle;

    #[test]
    fn test_deserialize() -> Result<(), String> {
        let data = r#"{
            "title": "Example Game",
            "description": "Example Description",
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Member 1",
                            "race": "Human",
                            "base_health": 10,
                            "attacks": [
                                {
                                    "name": "Kick",
                                    "base_damage": 123
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let battle: Battle = Battle::parse_from_str(data)?;
        assert_eq!(battle.teams[0].members[0].attacks[0].base_damage, 123);

        Ok(())
    }

    #[test]
    fn test_multi_player_error() {
        let data = r#"{
            "title": "Example Game",
            "description": "Example Description",
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Member 1",
                            "is_player": true,
                            "race": "Human",
                            "base_health": 10,
                            "attacks": []
                        },
                        {
                            "name": "Member 2",
                            "is_player": true,
                            "race": "Human",
                            "base_health": 10,
                            "attacks": []
                        }
                    ]
                }
            ]
        }"#;

        let maybe_battle = Battle::parse_from_str(data);

        assert!(maybe_battle.is_err());
        assert_eq!(
            maybe_battle.unwrap_err(),
            "Multiple playable team members found."
        );
    }
}
