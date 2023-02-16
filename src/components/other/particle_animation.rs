use gloo_timers::callback::Timeout;
use js_sys::Math;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

pub struct ParticleAnimation {
    canvas: Option<CanvasRenderingContext2d>,
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
            generator: ParticleGenerator::new(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
            ),
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let width = self.width.to_string();
        let height = self.height.to_string();
        let callback = {
            let link = ctx.link().clone();
            Callback::from(move |_| link.send_message(Msg::Rerender))
        };
        html! {
            <canvas id="canvas" width={width} height={height} onmousemove={callback}>
                {"Your browser does not support the HTML5 canvas tag."}
            </canvas>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas = document.get_element_by_id("canvas").unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
            let context = canvas.get_context("2d").unwrap().unwrap();
            let context: CanvasRenderingContext2d = context.dyn_into().unwrap();
            self.canvas = Some(context);

            self.generator.generate(1500);
            // start rendering
            ctx.link().send_message(Msg::Render);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render => {
                // log::info!("rendering");
                web_sys::console::log_1(&JsValue::from_str("rendering"));
                let context = self.canvas.as_ref().unwrap();

                context.clear_rect(0.0, 0.0, self.width.into(), self.height.into());

                let particles = self.generator.render();
                for (x, y) in particles {
                    // dots at x, y
                    context.begin_path();
                    context
                        .arc(x as f64, y as f64, 1.0, 0.0, 2.0 * std::f64::consts::PI)
                        .unwrap();
                    context.set_fill_style(&JsValue::from_str("white"));
                    context.fill();
                }

                self.generator.vibration();

                let link = ctx.link().clone();

                let timeout = Timeout::new(50, move || {
                    link.send_message(Msg::Render);
                });

                timeout.forget();

                true
            }
            Msg::Rerender => {
                web_sys::console::log_1(&JsValue::from_str("rendering"));
                true
            }
        }
    }
}

struct ParticleGenerator {
    particles: Vec<(u32, u32)>,
    width: u32,
    height: u32,
}

impl ParticleGenerator {
    fn new(width: u32, height: u32) -> Self {
        Self {
            particles: Vec::new(),
            width,
            height,
        }
    }

    fn generate(&mut self, n: u32) {
        for _ in 0..n {
            let x = Math::random() * self.width as f64;
            let y = Math::random() * self.height as f64;
            self.particles.push((x as u32, y as u32));
        }
    }

    fn vibration(&mut self) {
        for (x, y) in self.particles.iter_mut() {
            let rand = Math::random();
            if rand < 0.25 {
                *x += 1;
            } else if rand < 0.5 {
                if *x > 0 {
                    *x -= 1;
                }
            } else if rand < 0.75 {
                *y += 1;
            } else {
                if *y > 0 {
                    *y -= 1;
                }
            }
        }
    }

    fn render(&self) -> Vec<(u32, u32)> {
        self.particles.clone()
    }
}
