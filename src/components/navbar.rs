use yew::{function_component, html, Html, Properties};
use yew_router::prelude::Link;

use crate::router::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct NavbarProps {
    pub title: String,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    html! {
        <nav class="px-2 sm:px-4 py-2.5 shadow-xl bg-blue-200/50 comfortaa">
        <div class="flex flex-wrap items-center justify-between px-2 gap-4">
            <Link<Route> to={Route::Home}>
                <span
                    class="text-xl font-semibold font-logo text-blue-700 select-none"
                >
                    {&props.title }
                </span>
            </Link<Route>>
        </div>
    </nav>

    }
}
