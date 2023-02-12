use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div class="flex container mx-auto">
                <span class="text-6xl"> {"Welcome 2 da blog"} </span>
                <span class="text-xl "> {"This is a WIP"} </span>
            </div>
        </>
    }
}
