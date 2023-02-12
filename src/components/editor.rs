use kml::KMLDocument;
use wasm_bindgen::JsCast;
use web_sys::{Document, EventTarget, HtmlTextAreaElement};
use yew::prelude::*;
pub struct KMLEditor {
    kml_text: String,
    generated_html: String,
}

pub enum Msg {
    InputChanged(String),
}

impl Component for KMLEditor {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let text = include_str!("../posts/hello_world.kml");
        Self {
            kml_text: text.to_string(),
            generated_html: match KMLDocument::new(&text) {
                Ok(html) => html.to_html(),
                Err(e) => e.to_string(),
            },
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged(text) => {
                self.kml_text = text;
                self.generated_html = match KMLDocument::new(&self.kml_text) {
                    Ok(html) => html.to_html(),
                    Err(_) => self.generated_html.clone(),
                };

                Document::get_element_by_id(
                    &web_sys::window().unwrap().document().unwrap(),
                    "render-area",
                )
                .unwrap()
                .set_inner_html(&self.generated_html);

                true
            }
        }
    }

    fn rendered(&mut self, _: &Context<Self>, first_render: bool) {
        if first_render {
            Document::get_element_by_id(
                &web_sys::window().unwrap().document().unwrap(),
                "render-area",
            )
            .unwrap()
            .set_inner_html(&self.generated_html);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let kml_text = self.kml_text.clone();

        let on_input_change = ctx.link().callback(|input: InputEvent| {
            let target: EventTarget = input.target().unwrap();
            let input: HtmlTextAreaElement = target.dyn_into().unwrap();
            let value = input.value();
            Msg::InputChanged(value)
        });

        html! {
            <div class="grid grid-flow-row md:container mx-auto">
                <div class="grid grid-flow-col grid-cols-4">
                    <div class="col-span-2 px-4 py-4 rounded-b-lg  h-[90vh] ">
                        <textarea id="editor" rows="8"
                            class="block text-sm h-full w-full border-0 bg-gray-900 focus:ring-0 text-white overflow-x-scroll overflow-y-scroll whitespace-pre"
                            oninput={on_input_change}
                            value={kml_text}
                            >
                        </textarea>
                    </div>
                    <div class="col-span-2 px-4 py-4 rounded-b-lg  h-[90vh]">
                       <div id="render-area">
                       </div>
                    </div>
                </div>
            </div>
        }
    }
}
