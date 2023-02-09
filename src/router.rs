use yew::{html, Html};
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{"Coming Soon"}</h1>},
    }
}
