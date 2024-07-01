use actix_web::{
    dev::{ServerHandle, ServiceResponse},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpServer,
};
use futures::executor::block_on;
use std::{
    marker::PhantomData,
    sync::Arc,
    thread::{self, JoinHandle},
};
use tokio::sync::Mutex;

use crate::web_actor::handlers::{handle_act, handle_info, handle_sse};

pub struct Server<T> {
    _phantom: PhantomData<T>,
    server_thread: Option<JoinHandle<std::io::Result<()>>>,
    server_handle: ServerHandle,
}

#[actix_web::main]
async fn server_main<T>(server: actix_web::dev::Server) -> std::io::Result<()> {
    server.await
}

impl<T: Sync + Send + 'static> Server<T> {
    pub fn new(server_state: Arc<Mutex<T>>) -> Result<Server<T>, std::io::Error>
where {
        let host = "0.0.0.0";
        let port = 8000;
        const STATIC_HOSTING_DIR: &str = concat!(env!("OUT_DIR"), "/static");

        let server: actix_web::dev::Server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(server_state.clone()))
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
                .service(actix_files::Files::new("/", STATIC_HOSTING_DIR))
        })
        .disable_signals()
        .bind((host, port))
        .unwrap()
        .run();
        let server_handle = server.handle();
        println!("Started server on http://{}:{}/index.html", host, port);
        println!("Serving static assets from {}", STATIC_HOSTING_DIR);
        let server_thread = thread::spawn(|| server_main::<T>(server));

        Ok(Self {
            _phantom: PhantomData,
            server_thread: Some(server_thread),
            server_handle,
        })
    }
}

impl<T> Drop for Server<T> {
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
