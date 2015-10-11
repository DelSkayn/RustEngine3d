#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate cgmath;
#[macro_use]
extern crate log;
extern crate image;
extern crate time;

pub mod console;
pub mod event;
pub mod window;
pub mod engine;
pub mod math;
pub mod mesh;
pub mod obj;
pub mod camera;
pub mod render;


trait System{
    fn get_id() -> &'static str;
}
