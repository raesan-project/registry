use crate::{error, server, templates};
use askama::Template;
use axum::{self, response::IntoResponse};
use color_eyre::eyre::WrapErr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn index_route(
    axum::extract::State(web_state): axum::extract::State<Arc<RwLock<server::WebState>>>,
) -> error::HandlerResult<impl IntoResponse> {
    let web_state = web_state.write().await;
    println!("{:#?}", web_state);

    let html = templates::IndexRouteTemplate::builder()
        .build()
        .render()
        .wrap_err("failed to render file page HTML template")?;

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        html,
    )
        .into_response());
}
