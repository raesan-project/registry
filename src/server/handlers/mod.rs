use crate::{error, registry, server};
use axum::{self, response::IntoResponse};
use leptos::prelude::RenderHtml;
use mime_guess;
use serde_json;
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

    let html = server::web::pages::index_page::IndexPage(web::pages::index_page::IndexPageProps {
        registry_map,
    })
    .to_html();

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
    axum::extract::State(web_context): axum::extract::State<Arc<RwLock<server::WebContext>>>,
) -> error::HandlerResult<impl IntoResponse> {
    let web_context = web_context.write().await;

    let chapter_entry = match chapter_slug.as_str() {
        "magnetic-effect-of-current" => {
            if exam_slug == String::from("jee-main") {
                serde_json::json!("magnetics.json")
            } else {
                serde_json::json!(format!("{}.json", chapter_slug))
            }
        }
        _ => serde_json::json!(format!("{}.json", chapter_slug)),
    };

    let curr_chapter: registry::reg_models::Chapter = registry::map_chapter()
        .chapter_entry(&chapter_entry)
        .subject_entry_path_string(format!(
            "{}/{}/{}",
            web_context.registry_path, exam_slug, subject_slug
        ))
        .map_questions(true)
        .call()?;

    let html = server::web::pages::chapter_page::ChapterPage(
        server::web::pages::chapter_page::ChapterPageProps {
            chapter: curr_chapter,
        },
    )
    .to_html();

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        html,
    )
        .into_response());
}
