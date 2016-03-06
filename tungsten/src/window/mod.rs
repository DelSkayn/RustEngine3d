extern crate glutin;

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
    pub fn new(root: &Root) -> Self{
        let window = WindowBuilder::new()
            .with_dimensions(root.settings.graphics.window_size[0] as u32
                            ,root.settings.graphics.window_size[1] as u32)
            .with_title(root.settings.graphics.window_title.clone())
            .build().unwrap();
            window.set_position(root.settings.graphics.window_pos[0] as i32
                                ,root.settings.graphics.window_pos[0] as i32);
            WindowSystem{
                internal: window,
            }
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
