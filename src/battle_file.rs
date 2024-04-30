use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Battle {
    pub title: String,
    pub description: String,
    pub teams: Vec<Team>,
}

impl Battle {
    pub fn parse_from_str(data: &str) -> Result<Self, String> {
        serde_json::from_str(data).map_err(|err| err.to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub members: Vec<TeamMember>,
}

#[derive(Serialize, Deserialize)]
pub struct TeamMember {
    pub name: String,
    pub base_health: i64,
    pub attacks: Vec<Attack>,
}

#[derive(Serialize, Deserialize)]
pub struct Attack {
    pub name: String,
    pub base_damage: u16,
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
}
