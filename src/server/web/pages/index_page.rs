use crate::registry;
use crate::{server::web::pages::BaseLayout, utils};
use leptos::prelude::*;

#[component]
pub fn IndexPage(registry_map: Vec<registry::reg_models::Exam>) -> impl IntoView {
    let mut exams: Vec<AnyView> = Vec::new();
    for curr_exam in registry_map.iter() {
        let mut subjects: Vec<AnyView> = Vec::new();
        for curr_subject in curr_exam.subjects.iter() {
            let mut chapters: Vec<AnyView> = Vec::new();
            for curr_chapter in curr_subject.chapters.iter() {
                let href_slug = format!(
                    "/exam/{}/{}/{}",
                    utils::path_to_slug(curr_exam.name.clone()),
                    utils::path_to_slug(curr_subject.name.clone()),
                    utils::path_to_slug(curr_chapter.name.clone())
                );
                chapters.push(
                    view! {
                        <a href={href_slug} class="uk-card uk-card-body hover:uk-link hover:bg-muted transition-all">
                          <h3 class="uk-card-title">{curr_chapter.name.clone()}</h3>
                        </a>
                    }
                    .into_any(),
                );
            }
            subjects.push(
                view! {
                    <div class="flex gap-[10px] flex-col items-start">
                        <p class="text-2xl">{curr_subject.name.clone()}</p>
                        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 p-5 w-full">
                        {chapters}
                        </div>
                    </div>
                        <hr class="uk-divider-icon w-full" />
                }
                .into_any(),
            );
        }
        let curr_exam_modal_id = format!("modal-{}", utils::path_to_slug(curr_exam.name.clone()));
        let curr_exam_modal_id_href = format!("#{}", curr_exam_modal_id);
        exams.push(
            view! {
                <a href={curr_exam_modal_id_href} data-uk-toggle class="uk-card uk-card-body hover:uk-link hover:bg-muted transition-all">
                  <h3 class="uk-card-title">{curr_exam.name.clone()}</h3>
                </a>
                <div id={curr_exam_modal_id} class="uk-modal-full" data-uk-modal>
                  <div class="uk-modal-dialog uk-modal-body">
                    <button
                      class="uk-modal-close absolute right-[15px] top-[15px] p-3 uk-btn uk-btn-icon uk-btn-primary rounded-full"
                      type="button"
                      data-uk-close
                    ></button>
                    <h3 class="text-3xl">{curr_exam.name.clone()}</h3>
                    <div class="flex flex-col gap-[25px] p-5 w-full">
                    {subjects}
                    </div>
                  </div>
                </div>
            }
            .into_any(),
        );
    }
    view! {
        <BaseLayout>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 p-3 w-full">
                {exams}
            </div>
        </BaseLayout>
    }
}
