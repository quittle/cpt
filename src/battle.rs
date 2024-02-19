use crate::*;

type TeamId = u64;

pub struct ActionResult {
    pub description: String,
}

pub struct Team {
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
        for (team_id, actor) in &self.actors {
            ret.push(Turn {
                character: actor.get_character_id(),
            });
        }
        return ret;
    }

    fn get_actor(&self, character_id: CharacterId) -> Option<&Box<dyn Actor>> {
        for (team_id, actor) in &self.actors {
            if actor.get_character_id() == character_id {
                return Some(actor);
            }
        }
        return None;
    }

    fn require_actor(&self, character_id: CharacterId) -> &Box<dyn Actor> {
        self.get_actor(character_id).expect(&format!(
            "Unable to find actor with character id: {character_id}"
        ))
    }

    pub async fn advance(&mut self) {
        let turns = self.build_turns();
        for turn in turns {
            let action_result = self
                .require_actor(turn.character)
                .act(self, turn.character)
                .await;
            println!("Result {}", action_result.description);

            // turn.action(&mut self, turn.character);
            // println!("Turn {}", turn.action);
            // turn.action(turn.character);
        }
        // for &mut character in self.characeters {}
    }
}
