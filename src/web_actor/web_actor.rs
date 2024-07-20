use super::{
    handlers::{ArcEventSender, BattleServerEvent, ServerState},
    server::Server,
};
use crate::{ActionResult, Actor, Battle, CharacterId};
use actix_web_lab::sse;
use async_trait::async_trait;
use serde::Serialize;
use std::{path::Path, sync::Arc};
use tokio::sync::{
    mpsc::{channel, error::SendError, Receiver},
    Mutex,
};

#[derive(Serialize)]
struct BattleState<'battle> {
    battle: &'battle Battle,
    character_id: CharacterId,
}
pub struct WebActor {
    character_id: CharacterId,
    #[allow(dead_code)] // Required to stay alive during lifetime of WebActor
    server: Server<ServerState>,
    event_tx: ArcEventSender,
    action_rx: Arc<Mutex<Receiver<BattleServerEvent>>>,
}

impl WebActor {
    pub async fn new(
        character_id: CharacterId,
        additional_asset_directory: Option<&Path>,
    ) -> Result<Self, std::io::Error> {
        let (action_tx, action_rx) = channel(1);

        let event_tx = ArcEventSender::default();

        let server = Server::new(
            Arc::new(Mutex::new(ServerState {
                event_tx: event_tx.clone(),
                action_tx: action_tx.clone(),
            })),
            additional_asset_directory,
        )?;

        Ok(Self {
            character_id,
            server,
            event_tx,
            action_rx: Arc::new(Mutex::new(action_rx)),
        })
    }

    async fn send_battle_state(&self, battle: &Battle) -> Result<(), SendError<sse::Event>> {
        if let Some(sender) = self.event_tx.lock().await.as_ref() {
            sender
                .send(
                    sse::Data::new_json(BattleState {
                        battle,
                        character_id: self.character_id,
                    })
                    .unwrap()
                    .event("battle_state")
                    .into(),
                )
                .await?;
        }
        Ok(())
    }
}

#[async_trait]
impl Actor for WebActor {
    fn get_character_id(&self) -> &CharacterId {
        &self.character_id
    }

    async fn act(&self, battle: &Battle) -> ActionResult {
        self.send_battle_state(battle).await?;
        loop {
            match self.action_rx.lock().await.recv().await {
                Some(BattleServerEvent::BattleRequest) => {
                    self.send_battle_state(battle).await?;
                }
                Some(BattleServerEvent::Action(action)) => {
                    return action;
                }
                None => {}
            }
        }
    }

    async fn on_game_over(&self, battle: &Battle) {
        self.send_battle_state(battle)
            .await
            .expect("Failed to send game over state");
    }
}
