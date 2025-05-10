use crate::registry;
use leptos::prelude::*;

#[component]
pub fn BaseHTML(children: Children) -> impl IntoView {
    view! {
        <!doctype html>
        <html lang="en" class="dark uk-theme-raesan">
            <head>
                <title>raesan-registry</title>
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="stylesheet" href="/static/style.css" />
                <script src="/static/franken-ui.core.iife.js" type="module"></script>
                <script src="/static/franken-ui.icon.iife.js" type="module"></script>
            </head>
            <body class="bg-background text-foreground">
            {children()}
            </body>
        </html>
    }
}

pub fn path_to_slug(input: String) -> String {
    let mut output: String = String::new();
    let mut input_iter = input.chars().peekable();
    while let Some(curr_char) = input_iter.peek() {
        match curr_char {
            '&' => {
                output.push_str("and");
                input_iter.next();
            }
            _ => {
                if curr_char.is_whitespace() {
                    output.push('-');
                    input_iter.next();
                } else {
                    output.push(input_iter.next().unwrap());
                }
            }
        }
    }
    return output.to_lowercase();
}

#[component]
pub fn IndexPage(registry_map: Vec<registry::reg_models::Exam>) -> impl IntoView {
    // {sub_files}
    let mut chapters: Vec<AnyView> = Vec::new();
    for curr_exam in registry_map.iter() {
        for curr_subject in curr_exam.subjects.iter() {
            for curr_chapter in curr_subject.chapters.iter() {
                let href_slug = format!(
                    "/exam/{}/subject/{}/chapter/{}",
                    path_to_slug(curr_exam.name.clone()),
                    path_to_slug(curr_subject.name.clone()),
                    path_to_slug(curr_chapter.name.clone())
                );
                chapters.push(
                    view! {
                        <a class="uk-link" href={href_slug}>{curr_exam.name.clone()} - {curr_subject.name.clone()} - {curr_chapter.name.clone()}</a>
                    }
                    .into_any(),
                )
            }
        }
    }
    view! {
        <BaseHTML>
            <p class="text-2xl">Index Page</p>
            <div class="flex flex-col items-center gap-[10px]">
                {chapters}
            </div>
        </BaseHTML>
    }
}

#[component]
pub fn ChapterPage() -> impl IntoView {
    view! {
        <BaseHTML>
            <p class="text-2xl">Chapter Page</p>
        </BaseHTML>
    }
}
