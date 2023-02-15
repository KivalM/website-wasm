use yew::{function_component, html, Html};

use crate::components::navbar::Navbar;

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    html! {
        html! {
            <>
                <Navbar title="Projects"/>

            </>
        }
    }
}
