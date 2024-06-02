use crate::*;

use async_trait::async_trait;

#[async_trait]
pub trait Actor: Sync {
    /// Gets the character that this actor represents
    fn get_character_id(&self) -> &CharacterId;

    /// Perform action on turn
    async fn act(&self, battle: &Battle) -> ActionResult;

    /// Called when the game is over
    async fn on_game_over(&self, battle: &Battle);
}
