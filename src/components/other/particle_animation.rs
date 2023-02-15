use js_sys::Math;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

pub struct ParticleAnimation {
    canvas: Option<HtmlCanvasElement>,
    generator: ParticleGenerator,
    width: u32,
    height: u32,
}

pub enum Msg {
    Render,
    Rerender,
}

impl Component for ParticleAnimation {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let main_box = document.get_element_by_id("main-box").unwrap();

        let height = main_box.client_height();
        let width = main_box.client_width();

        Self {
            canvas: None,
            generator: ParticleGenerator::new(),
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let width = self.width.to_string();
        let height = self.height.to_string();
        html! {
            <canvas id="canvas" width={width} height={height}  >
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

                self.generator.generate(500, width, height);
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
            // draw random dots
            let x = Math::random() * width as f64;
            let y = Math::random() * height as f64;
            self.particles.push((x as u32, y as u32));
        }
    }

    fn render(&self) -> Vec<(u32, u32)> {
        self.particles.clone()
    }
}
