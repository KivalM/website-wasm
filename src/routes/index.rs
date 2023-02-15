use crate::components::{index::IndexMainComponent, navbar::Navbar};
use yew::{function_component, html, Html};

#[function_component(IndexPage)]
pub fn index_cta() -> Html {
    html! {
    <>
        <IndexMainComponent/>
        <Navbar title="Kival Mahadew"/>
    </>
    }
}
