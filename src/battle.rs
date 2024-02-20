use crate::*;

DeclareWrappedType!(TeamId, id, u64);

pub struct Team {
    pub name: String,
    pub id: TeamId,
}

struct Turn {
    character: CharacterId,
}

pub struct Battle {
    pub actors: Vec<(TeamId, Box<dyn Actor>)>,
    pub teams: Vec<Team>,
    // TODO, reordering of teams
}

impl Battle {
    fn build_turns(&self) -> Vec<Turn> {
        let mut ret = vec![];
        for (_team_id, actor) in &self.actors {
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

    pub async fn advance(&mut self) {
        let turns = self.build_turns();
        for turn in turns {
            let action_result = self.require_actor(turn.character).act(self).await;
            match action_result {
                Ok(request) => match request.action {
                    Action::Pass => {}
                    Action::AttackCharacter(target, damage) => {
                        self.require_mut_actor(target)
                            .damage(turn.character, damage);
                    }
                },
                Err(failure) => {
                    println!("Error processing {}: {}", turn.character, failure.message);
                }
            }
        }
    }
}
