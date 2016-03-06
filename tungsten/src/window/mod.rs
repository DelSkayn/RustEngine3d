extern crate glutin;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate gfx_core;

use self::gfx_core::format::Rgba8;

use self::gfx_device_gl::{
    Factory,
    Device,
};

use super::Root;

use super::kernal::System;
use super::kernal::Schedular;

use self::glutin::Window;
use self::glutin::Event;
use self::glutin::WindowBuilder;

pub struct WindowSystem{
    internal: Window,
}

impl WindowSystem{
    pub fn new(root: &Root) -> (Self,Device,Factory){
        let builder = WindowBuilder::new()
            .with_dimensions(root.settings.graphics.window_size[0] as u32
                            ,root.settings.graphics.window_size[1] as u32)
            .with_title(root.settings.graphics.window_title.clone())
            .with_visibility(false);

        let (window, device,factory,_,_) = 
            gfx_window_glutin::init::
            <Rgba8,gfx_core::format::Depth>(builder);


        window.set_position(root.settings.graphics.window_pos[0] as i32
                            ,root.settings.graphics.window_pos[0] as i32);
        window.show();
        (WindowSystem{
            internal: window,
        },device,factory)
    }
}

impl System for WindowSystem{
    fn run(&mut self,root: &Root,_: &mut Schedular){
        for event  in self.internal.poll_events(){
            match event{
                Event::Closed => {
                    root.running.quit();
                },
                _ => {},
            }
        }
    }
}
