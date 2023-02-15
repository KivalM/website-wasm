pub mod components;
pub mod router;
pub mod routes;

use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::router::{switch, Route};

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
