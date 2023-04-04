use yew::{function_component, html, Html};

#[function_component(CTA)]
pub fn cta() -> Html {
    html! {
        <div class="container mx-auto">
            <span>
                <span> {"coming soon."} </span>
            </span>
        </div>
    }
}
