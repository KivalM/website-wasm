use yew::{function_component, html, Html};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
    <nav class="px-2 sm:px-4 py-2.5 bg-slate-900">
        <div class="container flex flex-wrap items-center justify-between mx-auto">
            <div class="flex container mx-auto text-white">
                <span class="text-6xl"> {"The Blog"} </span>
                <span class="text-xl "> {"by Kival.M"} </span>
            </div>
        </div>
    </nav>
    }
}
