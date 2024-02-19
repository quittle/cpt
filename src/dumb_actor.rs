use crate::*;
use async_trait::async_trait;

pub struct DumbActor {
    pub character: Character,
}

#[async_trait]
impl Actor for DumbActor {
    async fn act(&self, _battle: &Battle, _character_id: CharacterId) -> ActionResult {
        Ok(ActionRequest {
            description: "attacking".into(),
        })
    }

    fn get_character_id(&self) -> CharacterId {
        self.character.id
    }
}
