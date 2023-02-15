use yew::{function_component, html, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct NavbarProps {
    pub title: String,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    html! {
        <nav class="container mx-auto bg-white h-20">
            <div class=" flex items-center h-full">
                // The page title is passed in as a prop
                <span class="text-6xl text-orange-600 ubuntu font-bold">
                    {&props.title}
                </span>

                // other navbar items
                <div class="flex-grow h-full">

                </div>
            </div>

            // Separator
            <div class="h-1 bg-orange-600 rounded-full"></div>
        </nav>
    }
}
