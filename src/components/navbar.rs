use yew::{function_component, html, Html, Properties};
use yew_router::prelude::{use_route, Link};

use crate::router::Route;

// #[derive(Clone, PartialEq, Properties)]
// pub struct NavbarProps {
// pub title: String,
// }

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let current_route = use_route::<Route>().unwrap();

    let standard_classes = "comfortaa font-black text-white";
    let active_classes = "comfortaa font-black text-[#E27D60]";

    html! {
    <>
        <div class="md:hidden flex items-center justify-center w-1/12">
            <svg class="w-6 h-6 text-[#E27D60]" fill="none" stroke-linecap="round" stroke-linejoin="round"
                stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
                <path d="M4 6h16M4 12h16M4 18h16"></path>
            </svg>
        </div>
        <nav class="bg-[#8338ec] p-5 hidden md:flex">
            <Link<Route> to={Route::Home}>
                <div class="comfortaa font-black text-white"> {"Kival M."} </div>
            </Link<Route>>

            <div class="flex flex-row gap-5 ml-auto">

                <Link<Route> to={Route::Home}>
                    <div class={ if current_route==Route::Home { active_classes } else { standard_classes } }> {"Home"}
                    </div>
                </Link<Route>>

                <Link<Route> to={Route::Blog}>
                    <div class={ if current_route==Route::Blog { active_classes } else { standard_classes } }> {"Blog"}
                    </div>
                </Link<Route>>

                <Link<Route> to={Route::Projects}>
                    <div class={ if current_route==Route::Projects { active_classes } else { standard_classes } }>
                        {"Projects"} </div>
                </Link<Route>>

                <Link<Route> to={Route::CV}>
                    <div class={ if current_route==Route::CV { active_classes } else { standard_classes } }> {"XP"}
                    </div>
                </Link<Route>>

            </div>
        </nav>
    </>

    }
}
