use crate::{cli, error, handlers};
use bon;
use std::sync::{Arc, RwLock};

// constants
pub const ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
pub const PORT: u16 = 8080;

// `WebState` struct
#[derive(Debug)]
pub struct WebState {}

#[bon::bon]
impl WebState {
    #[builder]
    pub fn new() -> Result<WebState, String> {
        return Ok(WebState {});
    }
}

#[bon::builder]
pub async fn serve(serve_data: cli::ServeQuestions) -> Result<(), error::ServerError> {
    let web_state = Arc::new(RwLock::new(match WebState::builder().build() {
        Ok(safe_app) => safe_app,
        Err(e) => {
            return Err(error::ServerError::WebStateError(e.to_string()));
        }
    }));

    println!("{:#?}", serve_data);

    let app_router: axum::Router = axum::Router::new()
        .route("/", axum::routing::get(handlers::index_route))
        .with_state(web_state);

    let listener =
        match tokio::net::TcpListener::bind(ADDRESS.to_string() + ":" + PORT.to_string().as_str())
            .await
        {
            Ok(safe_listener) => safe_listener,
            Err(e) => {
                eprintln!("Failed to bind TcpListener to address, Error: {:#?}", e);
                std::process::exit(1);
            }
        };

    println!("running on {}:{}", ADDRESS, PORT);

    match axum::serve(listener, app_router).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "Failed to start the server listener for the application, Error: {:#?}",
                e
            );
            std::process::exit(1);
        }
    };
    return Ok(());
}
