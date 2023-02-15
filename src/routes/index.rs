use crate::components::index::IndexMainComponent;
use yew::{function_component, html, Html};

#[function_component(IndexPage)]
pub fn index_cta() -> Html {
    html! {
    <>
        <IndexMainComponent/>
    </>
    }
}
