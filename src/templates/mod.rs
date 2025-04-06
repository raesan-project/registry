use askama::Template;
use bon;

#[derive(Template, bon::Builder)]
#[template(path = "index.html")]
pub struct IndexRouteTemplate {}
