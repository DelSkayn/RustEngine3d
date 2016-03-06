
<<<<<<< HEAD

struct RenderSystem{
}

struct RenderSystem
=======
use super::System;
use super::Root;
use super::AtomicOption;
use super::Schedular;

use super::window::WindowSystem;

pub struct RenderObject;

pub struct RenderRoot{
    render_list: AtomicOption<Vec<RenderObject>>,
}

impl RenderRoot{
    pub fn new() -> Self{
        RenderRoot{
            render_list: AtomicOption::new(),
        }
    }
}

mod gfx_renderer;
use self::gfx_renderer::GfxRenderer;

pub struct RenderSystem{
    renderer: GfxRenderer,
}

impl System for RenderSystem{
    fn run(&mut self,root: &Root,schedular: &mut Schedular){
    }
}
>>>>>>> c86e779063c9a631cac32eb6ffe9beb90778481f
