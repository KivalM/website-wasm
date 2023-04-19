use crate::components::{blog::cta::CTA, navbar::Navbar};
use yew::{function_component, html, Html};

#[function_component(BlogPage)]
pub fn blog_page() -> Html {
    html! {
        <>
            <Navbar />
            <CTA />
        </>
    }
}
