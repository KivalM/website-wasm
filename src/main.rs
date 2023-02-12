//! This is a simple single page application that renders a single component.
//! This will be the home page for the website.
//! Other info like CV/Blog etc will be registered on other subdomains.

pub mod components;
pub mod posts;
pub mod router;
pub mod routes;

use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::{
    components::navbar::Navbar,
    router::{switch, Route},
};

#[function_component(Main)]
fn app() -> Html {
    html! {
        <>
        <Navbar />
        <div class="min-h-screen bg-slate-800">
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
