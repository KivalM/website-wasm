use crate::router::Route;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, Document};
use yew::{function_component, html, Callback, Html};
use yew_router::prelude::{use_route, Link};

// #[derive(Clone, PartialEq, Properties)]
// pub struct NavbarProps {
// pub title: String,
// }

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let current_route = use_route::<Route>().unwrap();

    let standard_classes = "comfortaa font-black text-white";
    let active_classes = "comfortaa font-black text-[#E27D60]";

    let toggle_menu_cb = Callback::from(move |_| {
        let document = window()
            .expect_throw("window is undefined")
            .document()
            .expect_throw("document is undefined");

        let el = document.get_element_by_id("nav-items").unwrap_throw();
        let class_list = el.class_list();
        class_list.toggle("hidden").unwrap_throw();
    });

    html! {
    <>

        <nav class="bg-[#8338ec] p-5 flex flex-col md:flex-row">

            <div class="w-full flex justify-between">
                <Link<Route> to={Route::Home}>
                    <div class="comfortaa font-black text-white"> {"Kival M."} </div>
                </Link<Route>>
                <div class="flex-row gap-5 ml-auto flex md:hidden cursor-pointer" onclick={toggle_menu_cb}>
                    <svg class="w-6 h-6 text-white" fill="none" stroke-linecap="round" stroke-linejoin="round"
                        stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
                        <path d="M4 6h16M4 12h16M4 18h16"></path>
                    </svg>
                </div>
            </div>

            <div class="hidden flex-col md:ml-auto md:flex md:gap-5 md:flex-row" id="nav-items">
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
