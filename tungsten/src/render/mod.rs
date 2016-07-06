mod vulkan;
mod ogl;

use self::vulkan::Vulkan;
use self::ogl::Ogl;

#[derive(Debug)]
pub enum Error{
    ApiNotSupported,
    ApiVersionNotSupported,
    Other(&'static str),
}

trait Renderer{}


pub struct Render{
    renderer: Box<Renderer>,
}

impl Render{
    pub fn new() -> Self{
        let render: Box<Renderer> = match Vulkan::new(){
            Ok(x) => Box::new(x),
            Err(e) => {
                warn!("Could not init Vulkan renderer, Reason: {:?}",e);
                Box::new(Ogl::new().unwrap())
            },
        };
        Render{
            renderer: render,
        }
    }
}
