use crate::registry;
use crate::server::web::pages::BaseLayout;
use leptos::prelude::*;

#[component]
pub fn ChapterPage(chapter: registry::reg_models::Chapter) -> impl IntoView {
    let mut questions: Vec<AnyView> = Vec::new();
    for i in 0..chapter.questions.len() {
        match &chapter.questions[i] {
            registry::reg_models::Question::SingleMCQ(q) => {
                let mut options: Vec<AnyView> = Vec::new();
                for option in q.options.clone() {
                    options.push(
                        view! {
                            <div id="latex-body" class="flex gap-[5px]">
                                <p>{option.key}:</p>
                                <p id="latex-body" inner_html={option.value}></p>
                            </div>
                        }
                        .into_any(),
                    )
                }
                questions.push(
                    view! {
                        <div class="flex gap-[10px]" id={q.id.clone()}>
                            <p>{i + 1}.</p>
                            <div class="flex flex-col">
                                <p id="latex-body" inner_html={q.body.clone()}/>
                                <div class="flex flex-col gap-[5px]">
                                    {options}
                                </div>
                            </div>
                        </div>
                    }
                    .into_any(),
                )
            }
            registry::reg_models::Question::Numerical(q) => questions.push(
                view! {
                        <div class="flex gap-[10px]" id={q.id.clone()}>
                            <p>{i + 1}.</p>
                            <div class="flex flex-col">
                                <p id="latex-body" inner_html={q.body.clone()}/>
                            </div>
                        </div>
                }
                .into_any(),
            ),
        }
    }
    view! {
        <BaseLayout>
            <script src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/3.2.2/es5/tex-mml-chtml.min.js" defer></script>
            <script src="/static/scripts/render-mathjax.js"></script>
            <script src="/static/scripts/chapter-page.js"></script>
            <div class="p-3">
                <h2 class="text-3xl">{chapter.name}</h2>
                <div class="mb-[70px] mt-[30px] flex flex-col gap-[15px] px-[20px] overflow-x-auto overflow-y-hidden" id="question-list">
                    {questions}
                </div>
            </div>
        </BaseLayout>
    }
}
