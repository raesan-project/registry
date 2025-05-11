use crate::{cli, error};
use bon;
use color_eyre::eyre::{self, WrapErr};
use rust_embed;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing;

pub mod handlers;
pub mod web;

pub const ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
pub const PORT: u16 = 8080;
#[derive(rust_embed::Embed)]
#[folder = "static"]
pub struct StaticAssets;

pub fn get_embedded_file(filepath: String) -> eyre::Result<String, error::Error> {
    let file = StaticAssets::get(filepath.as_str()).ok_or_else(|| {
        error::Error::NotFound("no such embedded static file or directory".to_string())
    })?;
    let contents = String::from_utf8(file.data.to_vec())?;
    return Ok(contents);
}

#[derive(Debug, bon::Builder)]
pub struct WebContext {
    pub registry_path: String,
}

#[bon::builder]
pub async fn serve(server_data: cli::ServeQuestions) -> eyre::Result<()> {
    let web_state = Arc::new(RwLock::new(
        WebContext::builder()
            .registry_path(server_data.registry.clone())
            .build(), // .wrap_err("failed to create web state")?,
    ));

    let live_reload_layer = tower_livereload::LiveReloadLayer::new();
    let reloader = live_reload_layer.reloader();

    let router: axum::Router = axum::Router::new()
        .route("/", axum::routing::get(handlers::index_route))
        .route(
            "/static/*filepath",
            axum::routing::get(handlers::static_route),
        )
        .route(
            "/exam/:exam_slug/:subject_slug/:chapter_slug",
            axum::routing::get(handlers::chapter_route),
        )
        .with_state(web_state)
        .layer(live_reload_layer);

    let mut watcher =
        hotwatch::Hotwatch::new_with_custom_delay(std::time::Duration::from_millis(100))
            .wrap_err("failed to create a hotwatch instance with custom delay")?;
    watcher
        .watch(
            server_data.registry,
            move |event: hotwatch::Event| match event.kind {
                hotwatch::EventKind::Modify(hotwatch::notify::event::ModifyKind::Data(_))
                | hotwatch::EventKind::Create(_) => {
                    reloader.reload();
                }
                hotwatch::EventKind::Remove(_) => {}
                _ => {}
            },
        )
        .wrap_err("failed to watch for changes using hotwatch")?;

    let listener =
        tokio::net::TcpListener::bind(ADDRESS.to_string() + ":" + PORT.to_string().as_str())
            .await
            .wrap_err("failed to bind TCP Listener to address")?;

    tracing::info!("listening on {}:{}", ADDRESS.to_string(), PORT.to_string());

    axum::serve(listener, router)
        .await
        .wrap_err("Failed to start the serrver listener")?;
    return Ok(());
}
