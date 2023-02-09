use yew::prelude::*;

pub struct IndexPage {
    background_image: String,
}

pub enum Msg {
    OnClick,
}

impl Component for IndexPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            background_image: "https://images.unsplash.com/photo-1523590564318-491748f70ea7?ixlib=rb-4.0.3&dl=jack-b-rn-0OotfzFA-unsplash.jpg&w=2400&q=80&fm=jpg&crop=entropy&cs=tinysrgb".to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                // background image
                <div class={"bg-[url('https://images.unsplash.com/photo-1523590564318-491748f70ea7?ixlib=rb-4.0.3&dl=jack-b-rn-0OotfzFA-unsplash.jpg&w=2400&q=80&fm=jpg&crop=entropy&cs=tinysrgb')]
                            h-screen 
                            w-screen 
                            bg-cover 
                            bg-center
                        "}>
                    // dark overlay
                    <div class="bg-black bg-opacity-50 h-screen w-screen">
                        // content
                        <div class="h-screen w-screen flex flex-col justify-center items-center lupine text-center">
                            <div class="hover:scale-150 transition-all duration-100 cursor-pointer tracking-tight p-12">
                                <h1 class="text-4xl text-white font-bold hover:tracking-[0.25em] transition-all duration-100">{"i am"}</h1>
                                <h2 class="text-6xl text-white font-bold hover:tracking-[0.25em] transition-all duration-100">{"Kival Mahadew"}</h2>
                                <h3 class="text-xl text-white font-bold hover:tracking-[0.25em] transition-all duration-100">{"click me"}</h3>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
