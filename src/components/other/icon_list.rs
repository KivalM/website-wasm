use yew::{function_component, html, Html, Properties};

#[derive(Clone, PartialEq)]
pub struct Icon {
    pub name: String,
    pub link: String,
    pub icon: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub icons: Vec<Icon>,
}

#[function_component(IconList)]
pub fn icon_list(props: &Props) -> Html {
    html! {
        <>
            { for props.icons.iter().cloned().map(|icon| {
                html! {
                    <a href={icon.link} target="_blank" rel="noopener noreferrer">
                        <i class={format!("fab fa-{} fa-2x", &icon.icon)}></i>
                    </a>
                }
            })}
        </>
    }
}
