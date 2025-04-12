use crate::{server, templates};
use askama::Template;
use axum::{self, response::IntoResponse};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod error;

pub async fn index_route(
    axum::extract::State(web_state): axum::extract::State<Arc<RwLock<server::WebState>>>,
) -> error::HandlerResult<impl IntoResponse> {
    let web_state = web_state.write().await;
    println!("{:#?}", web_state);

    let html = templates::IndexRouteTemplate::builder()
        .build()
        .render()
        .map_err(|e| error::HandlerError::HTMLTemplateRender(e.to_string()))?;

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        html,
    )
        .into_response());
}
