use yew::prelude::*;

pub struct IndexPage {
    nav: bool,
}

pub enum Msg {
    OnClick,
}

impl Component for IndexPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { nav: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnClick => {
                self.nav = !self.nav;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::OnClick);
        html! {
        <>
            // background image
            <div class={"bg-[url('https://images.unsplash.com/photo-1523590564318-491748f70ea7?ixlib=rb-4.0.3&dl=jack-b-rn-0OotfzFA-unsplash.jpg&w=2400&q=80&fm=jpg&crop=entropy&cs=tinysrgb')]
                h-screen w-screen bg-cover bg-center "}>
                        // dark overlay
                        <div class=" bg-black bg-opacity-50 h-screen w-screen">

                <div class="select-none">
                if self.nav {
                // Greeter
                <div class="h-screen w-screen flex flex-col justify-center items-center lupine text-center" {onclick}>
                    <div class="hover:scale-150 transition-all duration-100 cursor-pointer tracking-tight p-12">
                        <h1 class="text-4xl text-white font-bold hover:tracking-[0.25em] transition-all duration-100">
                            {"i am"}</h1>
                        <h2 class="text-6xl text-white font-bold hover:tracking-[0.25em] transition-all duration-100">
                            {"Kival Mahadew"}</h2>
                        <h3 class="text-xl text-white font-bold hover:tracking-[0.25em] transition-all duration-100">
                            {"click me"}</h3>
                    </div>
                </div>

                } else{
                // Nav bar
                <div class="h-screen w-screen flex flex-col justify-center items-center lupine text-center text-6xl text-white gap-12">
                    <span class="hover:scale-125 transition-all duration-100 cursor-pointer hover:text-red-400"> {"Blog"} </span>
                    <span class="hover:scale-125 transition-all duration-100 cursor-pointer hover:text-red-400"> {"CV"} </span>
                    <span class="hover:scale-125 transition-all duration-100 cursor-pointer hover:text-red-400"> {"Projects"} </span>
                    <span class="hover:scale-125 transition-all duration-100 cursor-pointer hover:text-red-400"> {"Contact"} </span>
                    <span class="hover:scale-125 transition-all duration-100 cursor-pointer hover:text-red-400" {onclick}> {"Back"} </span>
                </div>
                }
                </div>
            </div>

            </div>

            // footer

            <div class="h-12 w-screen absolute bottom-0 text-gray-300 flex items-center justify-center">
                <div class="flex flex-row justify-center items-center">
                    {"Made in Rust with Wasm and Yew"}
                </div>
            </div>

        </>
        }
    }
}
