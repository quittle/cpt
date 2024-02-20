use crate::*;
use async_trait::async_trait;

pub struct DumbActor {
    pub character: Character,
}

#[async_trait]
impl Actor for DumbActor {
    async fn act(&self, _battle: &Battle) -> ActionResult {
        Ok(ActionRequest {
            description: "Doing Nothing".into(),
            action: Action::Pass,
        })
    }

    fn get_character(&self) -> &Character {
        &self.character
    }

    fn get_mut_character(&mut self) -> &mut Character {
        &mut self.character
    }
}
