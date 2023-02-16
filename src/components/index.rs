use crate::{components::other::particle_animation::ParticleAnimation, router::Route};
use yew::{function_component, html, Callback, Html};
use yew_router::prelude::use_navigator;

// mutable global variable to store the mouse position
pub static mut MOUSE_POS: (i32, i32) = (0, 0);

#[function_component(IndexMainComponent)]
pub fn index_main_component() -> Html {
    // add a mouse move callback to update the mouse position
    let mouse_move_callback = Callback::from(move |event: web_sys::MouseEvent| {
        let x = event.client_x();
        let y = event.client_y();

        unsafe {
            MOUSE_POS = (x, y);
        }
    });

    let touch_move_callback = Callback::from(move |event: web_sys::TouchEvent| {
        let touches = event.touches();
        let touch = touches.get(0).unwrap();
        let x = touch.client_x();
        let y = touch.client_y();

        unsafe {
            MOUSE_POS = (x, y);
        }
    });

    let navigator = use_navigator().unwrap();
    let navigate = move |route: Route| {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&route))
    };

    html! {
      <>
        <div class="h-screen w-screen bg-black p-2 sm:p-8 select-none">
            // inner box with w
            <div class="h-full w-full border-2 border-gray-400 rounded" id="main-box"
                onmousemove={mouse_move_callback} ontouchmove={touch_move_callback}>

                // box with background canvas that ignores other elements and is always behind everything
                <div class="absolute z-[0] overflow-hidden">
                    <ParticleAnimation />
                </div>

                // here we want 2 boxes for the right and left sides
                <div class="grid grid-cols-2 grid-flow-row p-6 h-full z-[1] relative">
                    // left box
                    <div class="float-left text-white  nueue-montreal-regular col-span-1">
                        <div class="text-4xl hover:underline"> {"Kival Mahadew"} </div>
                        <div class="text-3xl mt-2 hover:underline underline-offset-2">
                            {"CS Student & Software Developer"}
                        </div>

                        <div class="text-xl mt-4 hover:text-violet-400 transition-all" > {"- Home"} </div>
                        <div class="text-xl hover:text-violet-400 transition-all" onclick={navigate(Route::Blog)}> {"- Blog"} </div>
                        <div class="text-xl hover:text-violet-400 transition-all" onclick={navigate(Route::Projects)}> {"- Projects"} </div>
                    </div>

                    <div class="text-white text-xl nueue-montreal-regular col-span-1  flex flex-col justify-end">
                        <div class="flex flex-row justify-end">
                            <div class="flex flex-col justify-end">
                                // github icon
                                <div>
                                    <a class="inline-flex gap-2" href="mailto:kivalm@protonmail.com"
                                        referrer_policy="no-referrer" target="_blank">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                            class="bg-black text-white fill-current rounded-lg">
                                            <path
                                                d="M23 0l-4.5 16.5-6.097-5.43 5.852-6.175-7.844 5.421-5.411-1.316 18-9zm-11 12.501v5.499l2.193-3.323-2.193-2.176zm-8.698 6.825l-1.439-.507 5.701-5.215 1.436.396-5.698 5.326zm3.262 4.287l-1.323-.565 4.439-4.503 1.32.455-4.436 4.613zm-4.083.387l-1.481-.507 8-7.89 1.437.397-7.956 8z" />
                                        </svg>
                                        <span class="hover:text-violet-400 transition-all"> {"Email"} </span>
                                    </a>
                                </div>

                                <div>
                                    <a class="inline-flex gap-2" href="https://www.linkedin.com/in/kivalm/"
                                        referrer_policy="no-referrer" target="_blank">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                            class="bg-black text-white fill-current rounded-lg">

                                            <path
                                                d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" />
                                        </svg>
                                        <span class="hover:text-violet-400 transition-all"> {"LinkedIn"} </span>
                                    </a>
                                </div>

                                <div>
                                    <a class="inline-flex gap-2" href="https://github.com/kivalm/"
                                        referrer_policy="no-referrer" target="_blank">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                            class="bg-black text-white fill-current rounded-lg">
                                            <path
                                                d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
                                        </svg>
                                        <span class="hover:text-violet-400 transition-all"> {"Github"} </span>
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </>
      }
}
