#![allow(dead_code)]


#[macro_use]
extern crate glium;
//extern crate cgmath;
#[macro_use]
extern crate log;
extern crate image;
extern crate time;

pub mod console;
pub mod event;
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
    fn new(&render::RenderEngine) -> Self;
    fn render<'a>(&'a mut self,render::RenderQueue<'a>) -> render::RenderQueue<'a>;
    fn update(&mut self);
}


trait System{
    fn get_id() -> &'static str;
}
