use crate::{server, templates};
use askama::Template;
use axum::{self, response::IntoResponse};
use std::sync::{Arc, RwLock};

pub async fn index_route(
    axum::extract::State(web_state): axum::extract::State<Arc<RwLock<server::WebState>>>,
) -> Result<axum::response::Response, (axum::http::StatusCode, String)> {
    let web_state = match web_state.write() {
        Ok(safe_web_state) => safe_web_state,
        Err(e) => {
            println!("Failed to get web state, Error {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to get web state"),
            ));
        }
    };
    println!("{:#?}", web_state);

    let html = match (templates::IndexRouteTemplate {}.render()) {
        Ok(safe_html) => safe_html,
        Err(e) => {
            println!("Failed to render HTML template, Error: {:#?}", e);
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to render HTML template"),
            ));
        }
    };

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            String::from("text/html; charset=utf-8"),
        )],
        html,
    )
        .into_response());
}
