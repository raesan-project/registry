use crate::server::web::pages::BaseLayout;
use leptos::prelude::*;

#[component]
pub fn ErrorPage(status_code: String, error_message: String) -> impl IntoView {
    view! {
        <BaseLayout>
            <section>
                <div
                    class="container lg:min-h-screen mt-[100px] lg:mt-[0px] px-10 py-12 mx-auto lg:flex lg:items-center lg:gap-12"
                >
                    <div class="wf-ull lg:w-1/2">
                        <h1 class="mt-3 text-2xl font-semibold text-foreground md:text-3xl">Something went wrong!</h1>
                        <div class="mt-4 text-muted-foreground dark:text-gray-400 flex flex-col gap-[5px]">
                            <p>Error Code: <span class="uk-badge uk-badge-primary">{status_code}</span></p>
                            <p>Error Message: {error_message}</p>
                        </div>

                        <div class="flex items-center mt-6 gap-x-3">
                            <a class="uk-btn uk-btn-primary" href="/">Take me home</a>
                        </div>
                    </div>

                    <div class="relative w-full mt-12 lg:w-1/2 lg:mt-0">
                        <img class="w-full max-w-lg lg:mx-auto" src="/static/404.svg" alt="" />
                    </div>
                </div>
            </section>
        </BaseLayout>
    }
}
