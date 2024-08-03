use serde::{Deserialize, Serialize};

pub type LifeNumber = u64;
pub type CardId = usize;
pub type HandSize = u8;

#[derive(Serialize, Deserialize, Debug)]
pub struct Battle {
    pub title: String,
    pub description: String,
    pub board: Board,
    pub introduction: Option<StoryCard>,
    pub default_hand_size: HandSize,
    pub cards: Vec<Card>,
    pub teams: Vec<Team>,
}

impl Battle {
    pub fn parse_from_str(data: &str) -> Result<Self, String> {
        let battle: Battle = serde_json::from_str::<Battle>(data).map_err(|err| err.to_string())?;

        for (index, card) in battle.cards.iter().enumerate() {
            if card.id != index {
                return Err(format!("Card with id {} should be {}", card.id, index));
            }

            for action in &card.actions {
                let target = match action {
                    CardAction::Damage { target, .. } => target,
                    CardAction::Heal { target, .. } => target,
                    CardAction::GainAction { target, .. } => target,
                    CardAction::Move { target, .. } => target,
                };
                if target != &Target::Me && card.range.is_none() {
                    return Err(format!("Card with id {} has an action that can target others but without a range specified", card.id));
                }
            }
        }

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

pub type StoryCard = Vec<StoryCardEntry>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StoryCardEntry {
    H1(String),
    P(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    pub width: usize,
    pub height: usize,
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
    pub base_health: LifeNumber,
    pub max_health: Option<LifeNumber>,
    pub cards: Vec<CardId>,
    pub hand_size: Option<HandSize>,
    #[serde(default)]
    pub is_player: bool,
    pub image: Option<String>,
    pub location: (usize, usize),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Race {
    Human,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Target {
    #[serde(alias = "self")]
    Me,
    #[serde(alias = "other")]
    Others,
    #[serde(alias = "any")]
    Any,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum MaybeU64Range {
    Range(u64, u64),
    Absolute(u64),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CardAction {
    Damage {
        target: Target,
        amount: MaybeU64Range,
    },
    Heal {
        target: Target,
        amount: MaybeU64Range,
    },
    GainAction {
        target: Target,
        amount: MaybeU64Range,
    },
    Move {
        target: Target,
        amount: MaybeU64Range,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub description: String,
    pub flavor: Option<String>,
    pub actions: Vec<CardAction>,
    pub range: Option<u64>,
}

#[cfg(test)]
mod tests {
    use crate::battle_file::*;

    use super::Battle;

    #[test]
    fn test_deserialize() -> Result<(), String> {
        let data = r#"{
            "title": "Example Game",
            "description": "Example Description",
            "default_hand_size": 5,
            "introduction": [
                { "h1": "Heading" },
                { "p": "Paragraph" }
            ],
            "board": { "width": 1, "height": 1 },
            "cards": [
                {
                    "id": 0,
                    "name": "Kick",
                    "description": "description text",
                    "flavor": "flavor text",
                    "range": 1,
                    "actions": [
                        {
                            "type": "damage",
                            "target": "others",
                            "amount": 123
                        }
                    ]
                }
            ],
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Member 1",
                            "race": "Human",
                            "base_health": 10,
                            "cards": [0],
                            "location": [0, 0]
                        }
                    ]
                }
            ]
        }"#;

        let battle: Battle = Battle::parse_from_str(data)?;
        assert_eq!(
            battle.cards[battle.teams[0].members[0].cards[0]].actions[0],
            CardAction::Damage {
                target: Target::Others,
                amount: MaybeU64Range::Absolute(123),
            }
        );

        Ok(())
    }

    #[test]
    fn test_multi_player_error() {
        let data = r#"{
            "title": "Example Game",
            "description": "Example Description",
            "default_hand_size": 5,
            "cards": [],
            "board": { "width": 1, "height": 1 },
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Member 1",
                            "is_player": true,
                            "race": "Human",
                            "base_health": 10,
                            "cards": [],
                            "location": [0, 0]
                        },
                        {
                            "name": "Member 2",
                            "is_player": true,
                            "race": "Human",
                            "base_health": 10,
                            "cards": [],
                            "location": [0, 0]
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
