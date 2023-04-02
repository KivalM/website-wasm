// use gloo_timers::callback::Timeout;
// use js_sys::Math;
// use wasm_bindgen::{JsCast, JsValue};
// use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
// use yew::prelude::*;

// pub struct ParticleAnimation {
//     canvas: Option<CanvasRenderingContext2d>,
//     generator: ParticleGenerator,
//     offset: (u32, u32),
//     width: u32,
//     height: u32,
// }

// pub enum Msg {
//     Render,
// }

// impl Component for ParticleAnimation {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_: &Context<Self>) -> Self {
//         let document = web_sys::window().unwrap().document().unwrap();
//         let main_box = document.get_element_by_id("main-box").unwrap();

//         let height = main_box.client_height();
//         let width = main_box.client_width();

//         let div = main_box.dyn_into::<web_sys::HtmlDivElement>().unwrap();
//         let offset_top = div.offset_top();
//         let offset_left = div.offset_left();

//         Self {
//             canvas: None,
//             generator: ParticleGenerator::new(
//                 width.try_into().unwrap(),
//                 height.try_into().unwrap(),
//             ),
//             width: width.try_into().unwrap(),
//             height: height.try_into().unwrap(),
//             offset: (
//                 offset_left.try_into().unwrap(),
//                 offset_top.try_into().unwrap(),
//             ),
//         }
//     }

//     fn view(&self, _: &Context<Self>) -> Html {
//         let width = self.width.to_string();
//         let height = self.height.to_string();

//         html! {
//             <canvas id="canvas" width={width} height={height}>
//                 {"Your browser does not support the HTML5 canvas tag."}
//             </canvas>
//         }
//     }

//     fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
//         if first_render {
//             let document = web_sys::window().unwrap().document().unwrap();
//             let canvas = document.get_element_by_id("canvas").unwrap();
//             let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();
//             let context = canvas.get_context("2d").unwrap().unwrap();
//             let context: CanvasRenderingContext2d = context.dyn_into().unwrap();
//             self.canvas = Some(context);

//             let particles = {
//                 let density = 0.001;
//                 let square = self.width * self.height;
//                 (square as f64 * density) as usize
//             };
//             self.generator.add_particles(particles.try_into().unwrap());

//             // start rendering
//             ctx.link().send_message(Msg::Render);
//         }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::Render => {
//                 // log::info!("rendering");
//                 web_sys::console::log_1(&JsValue::from_str("rendering"));
//                 let context = self.canvas.as_ref().unwrap();

//                 context.clear_rect(0.0, 0.0, self.width.into(), self.height.into());

//                 let mouse = unsafe { MOUSE_POS };

//                 let mouse = (
//                     mouse.0 - self.offset.0 as i32,
//                     mouse.1 - self.offset.1 as i32,
//                 );

//                 self.generator.next(mouse.0.into(), mouse.1.into());
//                 let particles = self.generator.points();
//                 for (x, y) in particles {
//                     // dots at x, y
//                     context.begin_path();
//                     context
//                         .arc(x as f64, y as f64, 1.0, 0.0, 2.0 * std::f64::consts::PI)
//                         .unwrap();
//                     context.set_fill_style(&JsValue::from_str("white"));
//                     context.fill();
//                 }

//                 let link = ctx.link().clone();

//                 let fps = 60;
//                 let interval = 1000 / fps;

//                 let timeout = Timeout::new(interval, move || {
//                     link.send_message(Msg::Render);
//                 });

//                 timeout.forget();

//                 false
//             }
//         }
//     }
// }

// struct Particle {
//     x: f64,
//     y: f64,
//     vx: f64,
//     vy: f64,
// }

// struct ParticleGenerator {
//     particles: Vec<Particle>,
//     width: f64,
//     height: f64,
// }

// impl ParticleGenerator {
//     fn new(width: u32, height: u32) -> Self {
//         Self {
//             particles: Vec::new(),
//             width: width.into(),
//             height: height.into(),
//         }
//     }

//     fn add_particles(&mut self, n: u32) {
//         for _ in 0..n {
//             let x = Math::random() * self.width;
//             let y = Math::random() * self.height;
//             let vx = Math::random() * 2.0 - 1.0;
//             let vy = Math::random() * 2.0 - 1.0;

//             if vx < 0.5 && vx > -0.5 {
//                 // make sure the particle is moving
//                 if vx < 0.0 {
//                     -0.5
//                 } else {
//                     0.5
//                 };
//             }

//             if vy < 0.5 && vy > -0.5 {
//                 // make sure the particle is moving
//                 if vy < 0.0 {
//                     -0.5
//                 } else {
//                     0.5
//                 };
//             }

//             self.particles.push(Particle { x, y, vx, vy });
//         }
//     }

//     fn simulation(&mut self) {
//         for particle in self.particles.iter_mut() {
//             particle.x += particle.vx;
//             particle.y += particle.vy;

//             let elasticity = 0.9;

//             // bounce off the walls
//             if particle.x < 0.0 {
//                 particle.x = -particle.x;
//                 particle.vx = -particle.vx;
//                 particle.vx *= elasticity;
//             } else if particle.x >= self.width {
//                 particle.x = self.width - (particle.x - self.width);
//                 particle.vx = -particle.vx;
//                 particle.vx *= elasticity;
//             }

//             if particle.y < 0.0 {
//                 particle.y = -particle.y;
//                 particle.vy = -particle.vy;
//                 particle.vy *= elasticity;
//             } else if particle.y > self.height {
//                 particle.y = self.height - (particle.y - self.height);
//                 particle.vy = -particle.vy;
//                 particle.vy *= elasticity;
//             }
//         }
//     }

//     fn next(&mut self, x: f64, y: f64) {
//         // do all the normal stuff
//         self.simulation();

//         // handle the mouses
//         self.handle_attraction(x, y);
//     }

//     fn handle_attraction(&mut self, x: f64, y: f64) {
//         // particles will gain velocity towards the mouse
//         let attraction = 0.1;

//         for particle in self.particles.iter_mut() {
//             let dx = x - particle.x;
//             let dy = y - particle.y;
//             let distance = (dx * dx + dy * dy).sqrt();

//             if distance < 100.0 {
//                 let angle = dy.atan2(dx);
//                 particle.vx += attraction * angle.cos();
//                 particle.vy += attraction * angle.sin();
//             }
//         }
//     }

//     fn points(&self) -> Vec<(i32, i32)> {
//         self.particles
//             .iter()
//             .map(|p| (p.x as i32, p.y as i32))
//             .collect()
//     }
// }
