use js_sys::Math;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

pub struct ParticleAnimation {
    canvas: Option<HtmlCanvasElement>,
    generator: ParticleGenerator,
}

pub enum Msg {
    Render,
    Rerender,
}

impl Component for ParticleAnimation {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            canvas: None,
            generator: ParticleGenerator::new(),
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <canvas id="canvas" class="h-full w-full">
                {"Your browser does not support the HTML5 canvas tag."}
            </canvas>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas = document.get_element_by_id("canvas").unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
            self.canvas = Some(canvas);

            // start rendering
            ctx.link().send_message(Msg::Render);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render => {
                // log::info!("rendering");
                web_sys::console::log_1(&JsValue::from_str("rendering"));
                let canvas = self.canvas.as_ref().unwrap();
                let width = canvas.width();
                let height = canvas.height();
                let context = canvas.get_context("2d").unwrap().unwrap();
                let context: web_sys::CanvasRenderingContext2d = context.dyn_into().unwrap();

                context.clear_rect(0.0, 0.0, width as f64, height as f64);

                self.generator.generate(101, width, height);
                let particles = self.generator.render();

                for (x, y) in particles {
                    // dots at x, y
                    context.begin_path();
                    context.arc(x as f64, y as f64, 1.0, 0.0, 2.0 * std::f64::consts::PI);
                    context.set_fill_style(&JsValue::from_str("white"));
                    context.fill();
                }
                // ctx.link().send_message(Msg::Rerender);
                true
            }
            Msg::Rerender => {
                ctx.link().send_message(Msg::Render);
                true
            }
        }
    }
}

struct ParticleGenerator {
    particles: Vec<(u32, u32)>,
}

impl ParticleGenerator {
    fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    fn generate(&mut self, n: usize, width: u32, height: u32) {
        // generate n particles
        // in the shape of a circle
        for _ in 0..n {
            // use polar coordinates to generate points across a circle
            // let r = Math::random() * 100.0;
            // let theta = Math::random() * 2.0 * std::f64::consts::PI;
            // let x = (r * theta.cos()) as u32;
            // let y = (r * theta.sin()) as u32;
            // self.particles.push((x + width / 2, y + height / 2));
        }
    }

    fn render(&self) -> Vec<(u32, u32)> {
        self.particles.clone()
    }
}
