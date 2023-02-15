use yew::{function_component, html, Html};

#[function_component(BlogPage)]
pub fn blog_page() -> Html {
    html! {
        <div class="h-screen w-screen flex flex-col justify-center items-center text-center text-6xl text-black gap-12">
            <span>
                <span> {"Blog"} </span>
            </span>
        </div>
    }
}
