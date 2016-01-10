#![allow(dead_code)]


#[macro_use]
extern crate glium;
//extern crate cgmath;
#[macro_use]
extern crate log;
extern crate image;
extern crate time;

pub mod profile;

pub mod kernal;
pub mod console;
pub mod input;
pub mod window;
pub mod engine;
pub mod math;
pub mod obj;
pub mod render;
pub mod thread_pool;
pub mod resman;

const VERSION_MAJOR: &'static str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &'static str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &'static str = env!("CARGO_PKG_VERSION_PATCH");

pub trait Game{
    fn new() -> Self;
    fn render(&mut self);
    fn update(&mut self);
}

#[derive(Clone,Debug)]
pub enum Event{
    Profile(f64),
    Core(CoreEvent),
    Input(input::InputEvent),
    Render(render::RenderEvent),
}

#[derive(Clone,Debug)]
pub enum CoreEvent{
    Quit,
    Pause,
    Continue,
    Resize(u32,u32),
}
