use crate::*;

DeclareWrappedType!(TeamId, id, u64);

pub struct Team {
    pub id: TeamId,
    pub name: String,
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
        let turns = self.build_turns();
        for turn in turns {
            let actor = self.require_actor(turn.character);
            if actor.get_character().is_dead() {
                continue;
            }
            let action_result = actor.act(self).await;
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
            if !self.has_more_than_one_team_alive() {
                return;
            }
        }
    }
}
