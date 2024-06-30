use crate::{Action, ActionError, ActionFailure, ActionResult, Actor, Battle, CardId, CharacterId};
use actix_web::{
    dev::{Server, ServerHandle, ServiceResponse},
    get,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    post, web, App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::sse;
use async_trait::async_trait;
use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};
use tokio::sync::{
    mpsc::{channel, error::SendError, Receiver, Sender},
    Mutex,
};

type ArcEventSender = Arc<Mutex<Option<Sender<sse::Event>>>>;
type ArcServerState = Arc<Mutex<ServerState>>;

enum BattleServerEvent {
    Action(ActionResult),
    BattleRequest,
}

pub struct WebActor {
    character_id: CharacterId,
    server_thread: Option<JoinHandle<std::io::Result<()>>>,
    server_handle: ServerHandle,
    event_tx: ArcEventSender,
    action_rx: Arc<Mutex<Receiver<BattleServerEvent>>>,
}

struct ServerState {
    event_tx: ArcEventSender,
    action_tx: Sender<BattleServerEvent>,
}

#[derive(Deserialize)]
struct ActParams {
    card_id: usize,
    target_id: usize,
}

#[post("/act")]
async fn handle_act(
    info: web::Json<ActParams>,
    state: web::Data<ArcServerState>,
) -> impl Responder {
    state
        .lock()
        .await
        .action_tx
        .send(BattleServerEvent::Action(ActionResult::Ok(Action::Act(
            CardId::new(info.card_id),
            CharacterId::new(info.target_id),
        ))))
        .await
        .unwrap();
    HttpResponse::Ok()
}

#[get("/info")]
async fn handle_info(state: web::Data<ArcServerState>) -> impl Responder {
    state
        .lock()
        .await
        .action_tx
        .send(BattleServerEvent::BattleRequest)
        .await
        .unwrap();
    HttpResponse::Ok()
}

#[get("/sse")]
async fn handle_sse(state: web::Data<ArcServerState>) -> impl Responder {
    let (tx, rx) = channel(10);

    state.lock().await.event_tx.lock().await.replace(tx);

    sse::Sse::from_infallible_receiver(rx).with_retry_duration(Duration::from_secs(10))
}

#[actix_web::main]
async fn server_main(server: Server) -> std::io::Result<()> {
    server.await
}

impl WebActor {
    pub async fn new(character_id: CharacterId) -> Result<Self, std::io::Error> {
        let (action_tx, action_rx) = channel(1);

        let event_tx = ArcEventSender::default();
        let server_event_tx = event_tx.clone();

        let host = "0.0.0.0";
        let port = 8000;

        let server: actix_web::dev::Server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(Arc::new(Mutex::new(ServerState {
                    event_tx: server_event_tx.clone(),
                    action_tx: action_tx.clone(),
                }))))
                .wrap(
                    ErrorHandlers::new().default_handler(|service_response: ServiceResponse| {
                        println!(
                            "{}: {:?}",
                            service_response.status(),
                            service_response.response().body()
                        );
                        Ok(ErrorHandlerResponse::Response(
                            service_response.map_into_left_body(),
                        ))
                    }),
                )
                .service(handle_act)
                .service(handle_info)
                .service(handle_sse)
                .service(actix_files::Files::new(
                    "/",
                    concat!(env!("OUT_DIR"), "/static"),
                ))
        })
        .disable_signals()
        .bind((host, port))
        .unwrap()
        .run();
        let server_handle = server.handle();
        println!("Started server on http://{}:{}/index.html", host, port);
        let server_thread = thread::spawn(|| server_main(server));

        Ok(Self {
            character_id,
            server_thread: Some(server_thread),
            server_handle,
            event_tx,
            action_rx: Arc::new(Mutex::new(action_rx)),
        })
    }
}

impl Drop for WebActor {
    fn drop(&mut self) {
        block_on(self.server_handle.stop(true));
        self.server_thread
            .take()
            .expect("Server thread does not exist")
            .join()
            .expect("Failed to join the server thread")
            .expect("Server exited ungracefully.");
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

    async fn on_game_over(&self, _battle: &Battle) {}
}

#[derive(Serialize)]
struct BattleState<'battle> {
    battle: &'battle Battle,
    character_id: CharacterId,
}

impl WebActor {
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

impl From<SendError<sse::Event>> for ActionError {
    fn from(send_error: SendError<sse::Event>) -> Self {
        Self::Failure(ActionFailure {
            message: send_error.to_string(),
        })
    }
}
