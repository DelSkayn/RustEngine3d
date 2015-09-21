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

trait System{
    fn get_id() -> &'static str;
}
