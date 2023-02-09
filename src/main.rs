//! This is a simple single page application that renders a single component.
//! This will be the home page for the website.
//! Other info like CV/Blog etc will be registered on other subdomains.

pub mod router;

use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::router::{switch, Route};

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
