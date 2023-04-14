use yew::{html, Html};
use yew_router::Routable;

use crate::routes::{blog::BlogPage, cv::CVPage, index::IndexPage, projects::ProjectsPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/projects")]
    Projects,
    #[at("/cv")]
    CV,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <IndexPage/>
        },
        Route::Blog => html! {
            <BlogPage/>
        },

        Route::Projects => html! {
            <ProjectsPage/>
        },
        Route::CV => html! {
            <CVPage/>
        },

        Route::NotFound => {
            html! {
                <div class="h-screen w-screen flex flex-col justify-center items-center text-center text-6xl text-black gap-12">
                    <span>
                        <span> {"404"} </span>
                    </span>
                </div>
            }
        }
    }
}
