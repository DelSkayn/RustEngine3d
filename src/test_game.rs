use super::engine3d::Game;
use super::engine3d::obj;
use super::engine3d::render;
use super::engine3d::math::*;
use super::engine3d::render::camera;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct TestGame{
    temp: u8,
}

impl Game for TestGame{
    fn new() -> Self{
        TestGame{
            temp: 3,
        }
    }
    fn render(&mut self){

    }
    fn update(&mut self){

    }
}
