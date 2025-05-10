use crate::{error, registry, server};
use axum::{self, response::IntoResponse};
use leptos::prelude::RenderHtml;
use mime_guess;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::server::web;

pub async fn static_route(
    axum::extract::Path(filepath): axum::extract::Path<String>,
) -> error::HandlerResult<impl IntoResponse> {
    let file_contents = server::get_embedded_file(filepath.to_string())?;

    // get the file type
    let file_type = mime_guess::from_path(filepath.to_string()).first_or_octet_stream();

    return Ok((
        [(axum::http::header::CONTENT_TYPE, file_type.to_string())],
        file_contents,
    )
        .into_response());
}

pub async fn index_route(
    axum::extract::State(web_context): axum::extract::State<Arc<RwLock<server::WebContext>>>,
) -> error::HandlerResult<impl IntoResponse> {
    let web_context = web_context.write().await;

    let registry_map = registry::map_registry()
        .registry_path_string(web_context.registry_path.clone())
        .map_questions(false)
        .call()?;

    let html = server::web::IndexPage(web::IndexPageProps { registry_map }).to_html();

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        html,
    )
        .into_response());
}

pub async fn chapter_route(
    axum::extract::Path((exam_slug, subject_slug, chapter_slug)): axum::extract::Path<(
        String,
        String,
        String,
    )>,
) -> error::HandlerResult<impl IntoResponse> {
    tracing::info!("{:#?}", exam_slug);
    tracing::info!("{:#?}", subject_slug);
    tracing::info!("{:#?}", chapter_slug);
    return Ok((axum::Json("hello, world!")).into_response());
}
