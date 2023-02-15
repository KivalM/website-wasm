use yew::{function_component, html, use_state, Callback, Html};
use yew_router::prelude::use_navigator;

use crate::router::Route;

#[function_component(IndexMainComponent)]
pub fn index_main_component() -> Html {
    // visibility toggle
    let visible = use_state(|| true);
    let onclick = {
        let visible = visible.clone();
        Callback::from(move |_| visible.set(!*visible))
    };

    let navigator = use_navigator().unwrap();
    let navigate = move |route: Route| {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&route))
    };

    html! {
        <>
          // background image
          <div class={"bg-[url('https://images.unsplash.com/photo-1523590564318-491748f70ea7?ixlib=rb-4.0.3&dl=jack-b-rn-0OotfzFA-unsplash.jpg&w=2400&q=80&fm=jpg&crop=entropy&cs=tinysrgb')]
              h-screen w-screen bg-cover bg-center "}>
                          // dark overlay
                          <div class=" bg-black bg-opacity-50 h-screen w-screen">

              <div class="select-none">
                    // Greeter
                  if *visible {

                  <div class="h-screen w-screen flex flex-col justify-center items-center nueue-montreal-light text-center" {onclick}>
                      <div class="hover:scale-150 transition-all duration-100 cursor-pointer tracking-tight p-12">
                          <h1 class="text-4xl text-white font-bold hover:tracking-[0.25em] transition-all duration-100">
                              {"i am"}
                          </h1>
                          <h2 class="text-6xl text-white font-bold hover:tracking-[0.25em] transition-all duration-100">
                              {"Kival Mahadew"}
                          </h2>
                          <h3 class="text-xl text-white font-bold hover:tracking-[0.25em] transition-all duration-100">
                              {"click me !"}
                          </h3>
                      </div>
                  </div>

                  } else{
                  // Nav bar
                  <div
                      class="h-screen w-screen flex flex-col justify-center items-center lupine text-center text-6xl text-white gap-12">
                      <span onclick={navigate(Route::Blog)}
                          class="hover:scale-125 transition-all duration-100 cursor-pointer hover:text-red-400">
                          <span> {"Blog"} </span>
                      </span>

                      <span onclick={navigate(Route::Projects)}
                          class="hover:scale-125 transition-all duration-100 cursor-pointer hover:text-red-400">
                          <span> {"Projects"} </span>
                      </span>


                      <span onclick={navigate(Route::Projects)}
                          class="hover:scale-125 transition-all duration-100 cursor-pointer hover:text-red-400">
                          <span> {"Github"} </span>
                      </span>

                    <span onclick={navigate(Route::Projects)} class="hover:scale-125 transition-all duration-100 cursor-pointer hover:text-red-400">
                        <span> {"LinkedIn"} </span>
                    </span>

                      <span class="hover:scale-125 transition-all duration-100 cursor-pointer hover:text-red-400"
                          {onclick}> {"Back"} </span>
                  </div>
                  }
              </div>
          </div>
          </div>
      </>
    }
}
