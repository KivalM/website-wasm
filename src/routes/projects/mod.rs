use crate::{components::navbar::Navbar, routes::projects::models::PROJECTS};
use yew::{function_component, html, Html};

pub mod models;

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    html! {
    <>
        <Navbar />

        <div class="container mx-auto">
            <div class="text-[#fb5607] font-black text-2xl comfortaa py-4">
                {"Projects"}
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                {PROJECTS.iter().map(|f| f.to_html()).collect::<Html>()}
            </div>
        </div>


    </>
    }
}
