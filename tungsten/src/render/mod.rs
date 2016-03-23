use super::System;
use super::Root;
use super::util::AtomicOption;
use super::kernal::JobBuilder;

mod gfx_renderer;
pub use self::gfx_renderer::GfxRenderer;

mod data;
pub use self::data::*;

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
    fn load_mesh(&mut self,mesh: Mesh);
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
    fn run(&mut self,_root: &Root) -> Option<JobBuilder>{
        None
    }
}
