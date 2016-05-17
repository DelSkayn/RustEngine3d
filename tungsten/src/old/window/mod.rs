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

use super::kernel::System;
use super::kernel::TaskBuilder;

use self::glutin::Window as GluWindow;
use self::glutin::Event;
use self::glutin::WindowBuilder;

pub struct Context;

pub struct WindowData{
    context: Context,
}

pub struct Window{
    internal: GluWindow,
}

impl Window{
    pub fn new(root: &Root) -> Self{
        let builder = WindowBuilder::new()
            .with_dimensions(root.sync.settings.graphics.window_size[0] as u32
                            ,root.sync.settings.graphics.window_size[1] as u32)
            .with_title(root.sync.settings.graphics.window_title.clone())
            .with_visibility(false);

        let window = builder.build();

        window.set_position(root.sync.settings.graphics.window_pos[0] as i32
                            ,root.sync.settings.graphics.window_pos[0] as i32);
        window.show();
        Window{
            internal: window,
        }
    }
}


