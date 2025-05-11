use leptos::prelude::*;

pub mod chapter_page;
pub mod error_page;
pub mod index_page;

#[component]
pub fn BaseLayout(children: Children) -> impl IntoView {
    view! {
        <!doctype html>
        <html lang="en" class="dark uk-theme-raesan">
            <head>
                <title>raesan-registry</title>
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="stylesheet" href="/static/style.css" />
                <script
                  src="https://cdn.jsdelivr.net/npm/franken-ui@2.0.0/dist/js/core.iife.js"
                  type="module"
                ></script>
                <script
                  src="https://cdn.jsdelivr.net/npm/franken-ui@2.0.0/dist/js/icon.iife.js"
                  type="module"
                ></script>
            </head>
            <body class="bg-background text-foreground">
            {children()}
            </body>
        </html>
    }
}
