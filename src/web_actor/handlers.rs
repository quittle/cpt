use crate::{Action, ActionError, ActionFailure, ActionResult, CardId, CharacterId, GridLocation};
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_lab::sse;
use serde::Deserialize;
use std::{sync::Arc, time::Duration};
use tokio::sync::{
    mpsc::{channel, error::SendError, Sender},
    Mutex,
};

pub type ArcEventSender = Arc<Mutex<Option<Sender<sse::Event>>>>;
pub type ArcServerState = Arc<Mutex<ServerState>>;

pub enum BattleServerEvent {
    Action(ActionResult),
    BattleRequest,
}

pub struct ServerState {
    pub event_tx: ArcEventSender,
    pub action_tx: Sender<BattleServerEvent>,
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

#[derive(Deserialize)]
struct MoveDestination {
    x: usize,
    y: usize,
}

#[derive(Deserialize)]
struct MoveParams {
    target_id: usize,
    to: MoveDestination,
}

#[post("/move")]
async fn handle_move(
    info: web::Json<MoveParams>,
    state: web::Data<ArcServerState>,
) -> impl Responder {
    state
        .lock()
        .await
        .action_tx
        .send(BattleServerEvent::Action(ActionResult::Ok(Action::Move(
            CharacterId::new(info.target_id),
            GridLocation {
                x: info.to.x,
                y: info.to.y,
            },
        ))))
        .await
        .unwrap();
    HttpResponse::Ok()
}

#[post("/pass")]
async fn handle_pass(state: web::Data<ArcServerState>) -> impl Responder {
    state
        .lock()
        .await
        .action_tx
        .send(BattleServerEvent::Action(ActionResult::Ok(Action::Pass)))
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

impl From<SendError<sse::Event>> for ActionError {
    fn from(send_error: SendError<sse::Event>) -> Self {
        Self::Failure(ActionFailure {
            message: send_error.to_string(),
        })
    }
}
