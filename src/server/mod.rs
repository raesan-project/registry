use crate::{cli, handlers};
use bon;
use color_eyre::eyre;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing;

// constants
pub const ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
pub const PORT: u16 = 8080;

// `WebState` struct
#[derive(Debug)]
pub struct WebState {}

#[bon::bon]
impl WebState {
    #[builder]
    pub fn new() -> eyre::Result<WebState> {
        return Ok(WebState {});
    }
}

#[bon::builder]
pub async fn serve(serve_data: cli::ServeQuestions) -> eyre::Result<()> {
    let web_state = Arc::new(RwLock::new(WebState::builder().build()?));

    println!("{:#?}", serve_data);

    let app_router: axum::Router = axum::Router::new()
        .route("/", axum::routing::get(handlers::index_route))
        .with_state(web_state);

    let listener =
        tokio::net::TcpListener::bind(ADDRESS.to_string() + ":" + PORT.to_string().as_str())
            .await?;

    tracing::debug!("listening on {}:{}", ADDRESS.to_string(), PORT.to_string());

    axum::serve(listener, app_router).await?;
    return Ok(());
}
