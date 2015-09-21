use super::glium::backend::glutin_backend::{
    GlutinFacade,
};
use super::glium::DisplayBuild;

use super::glium::glutin::WindowBuilder;

use glium::glutin::Event;

use super::event::{
    EventCreator,
};

use super::event::BaseEvent::*;
use super::event::BaseEvent;

pub struct Window{
    window: GlutinFacade,
}

impl Window{
    pub fn new() -> Self{
        let builder = WindowBuilder::new().with_dimensions(800,600);
        Window{
            window: builder.build_glium().unwrap(),
        }
    }
}

impl EventCreator<BaseEvent> for Window{
    fn get_events(&mut self) -> Vec<BaseEvent>{
        self.window.poll_events().filter_map(
            |ev| match ev{
                Event::Resized(w,h) => Some(Resize(w,h)),
                Event::Closed => Some(Quit),
                _ => None,
            }).collect()
    }
}


