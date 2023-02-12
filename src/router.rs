use yew::{html, Html};
use yew_router::Routable;

use crate::{components::editor::KMLEditor, routes::home::Home};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/editor")]
    Editor,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::Editor => html!(<KMLEditor/>),
    }
}
