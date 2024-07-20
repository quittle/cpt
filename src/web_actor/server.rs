use actix_web::{
    dev::{ServerHandle, ServiceResponse},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpServer,
};
use futures::executor::block_on;
use std::{
    marker::PhantomData,
    path::Path,
    sync::Arc,
    thread::{self, JoinHandle},
};
use std::{
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
};
use tokio::sync::Mutex;

use crate::web_actor::handlers::{handle_act, handle_info, handle_sse};

pub struct Server<T> {
    _phantom: PhantomData<T>,
    server_thread: Option<JoinHandle<std::io::Result<()>>>,
    server_handle: ServerHandle,

    #[cfg(debug_assertions)]
    asset_build_thread: Option<JoinHandle<()>>,
    #[cfg(debug_assertions)]
    asset_build_thread_terminate: Arc<AtomicBool>,
}

#[actix_web::main]
async fn server_main<T>(server: actix_web::dev::Server) -> std::io::Result<()> {
    server.await
}

impl<T: Sync + Send + 'static> Server<T> {
    pub fn new(
        server_state: Arc<Mutex<T>>,
        additional_static_asset_directory: Option<&Path>,
    ) -> Result<Server<T>, std::io::Error>
where {
        let host = "0.0.0.0";
        let port = 8000;
        const STATIC_HOSTING_DIR: &str = concat!(env!("OUT_DIR"), "/static");
        let additional_static_asset_directory =
            additional_static_asset_directory.map(Path::to_path_buf);

        let server: actix_web::dev::Server = HttpServer::new(move || {
            let mut app = App::new()
                .app_data(web::Data::new(server_state.clone()))
                .wrap(
                    ErrorHandlers::new().default_handler(|service_response: ServiceResponse| {
                        println!(
                            "{} {}: {:?}",
                            service_response.request().path(),
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
                .service(handle_sse);
            if let Some(dir) = &additional_static_asset_directory {
                app = app.service(actix_files::Files::new("/ref", dir.clone()).use_etag(true));
            }
            // Must come after the additional directory to ensure resolution
            app = app.service(actix_files::Files::new("/", STATIC_HOSTING_DIR).use_etag(true));
            app
        })
        .disable_signals()
        .bind((host, port))
        .unwrap()
        .run();
        let server_handle = server.handle();
        println!("Started server on http://{}:{}/index.html", host, port);
        println!("Serving static assets from {}", STATIC_HOSTING_DIR);
        let server_thread = thread::spawn(|| server_main::<T>(server));

        #[cfg(debug_assertions)]
        {
            let asset_build_thread_terminate: Arc<AtomicBool> = Default::default();
            let thread_bool = asset_build_thread_terminate.clone();
            Ok(Self {
                _phantom: PhantomData,
                server_thread: Some(server_thread),
                server_handle,
                asset_build_thread: Some(thread::spawn(move || {
                    while !thread_bool.load(Ordering::Relaxed) {
                        if let Ok(status) = Command::new("npm")
                            .args(["run", "build-server"])
                            .env("OUT_DIR", env!("OUT_DIR"))
                            .status()
                        {
                            if !status.success() {
                                println!("Asset build failed!");
                            }
                        }
                    }
                })),
                asset_build_thread_terminate,
            })
        }
        #[cfg(not(debug_assertions))]
        {
            Ok(Self {
                _phantom: PhantomData,
                server_thread: Some(server_thread),
                server_handle,
            })
        }
    }
}

impl<T> Drop for Server<T> {
    fn drop(&mut self) {
        self.asset_build_thread_terminate
            .store(true, Ordering::Relaxed);
        self.asset_build_thread.take().unwrap().join().unwrap();
        block_on(self.server_handle.stop(true));
        self.server_thread
            .take()
            .expect("Server thread does not exist")
            .join()
            .expect("Failed to join the server thread")
            .expect("Server exited ungracefully.");
    }
}
