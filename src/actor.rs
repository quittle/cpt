use crate::*;

use async_trait::async_trait;

#[async_trait]
pub trait Actor {
    async fn act(&self, battle: &Battle, character_id: CharacterId) -> ActionResult;
    fn get_character_id(&self) -> CharacterId;
}
