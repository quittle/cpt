use crate::{
    battle_file, web_actor::WebActor, Actor, Battle, Card, CardAction, CardId, Character,
    CharacterId, CharacterRace, DumbActor, Health, LifeNumberRange, RandomProvider, Target, Team,
    TeamId, TerminalActor,
};
use futures::future::join_all;

fn normalize_maybe_life_number_range(
    life_number_range: &battle_file::MaybeLifeNumberRange,
) -> LifeNumberRange {
    match *life_number_range {
        battle_file::MaybeLifeNumberRange::Absolute(value) => LifeNumberRange(value, value),
        battle_file::MaybeLifeNumberRange::Range(low, high) => LifeNumberRange(low, high),
    }
}

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
            introduction: battle.introduction,
            random_provider,
            default_turn_actions: 1,
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
                            remaining_actions: 0,
                            deck: member
                                .cards
                                .iter()
                                .map(|card_id| CardId::new(*card_id))
                                .collect(),
                            health: Health::new(member.base_health),
                            max_health: Health::new(
                                member.max_health.unwrap_or(member.base_health),
                            ),
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
                        battle_file::Target::Any => Target::Any,
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
                                            amount: normalize_maybe_life_number_range(amount),
                                        }
                                    }
                                    battle_file::CardAction::Heal { target, amount } => {
                                        CardAction::Heal {
                                            target: map_target(target),
                                            amount: normalize_maybe_life_number_range(amount),
                                        }
                                    }
                                    battle_file::CardAction::GainAction { target, amount } => {
                                        CardAction::GainAction {
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
}
