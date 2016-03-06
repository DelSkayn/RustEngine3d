use super::System;
use super::Root;
use super::AtomicOption;
use super::Schedular;

mod gfx_renderer;
pub use self::gfx_renderer::GfxRenderer;

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

pub trait Renderer{
    fn render(&mut self);
}

pub struct RenderSystem<R: Renderer>{
    renderer: R,
}

impl<R: Renderer> RenderSystem<R>{
    pub fn new(renderer: R) -> Self{
        RenderSystem{
            renderer: renderer,
        }
    }
}

impl<R: Renderer> System for RenderSystem<R>{
    fn run(&mut self,_root: &Root,schedular: &mut Schedular){
    }
}
