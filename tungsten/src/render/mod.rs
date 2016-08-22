mod vulkan;
mod ogl;
mod format;

extern crate nalgebra;

use self::nalgebra::{Perspective3,UnitQuaternion,Vector3};

pub use self::format::*;

use self::vulkan::Vulkan;
use self::ogl::Ogl;

use registery::Registery;
pub use window::WindowContext;

#[derive(Debug)]
pub enum Error{
    ApiNotSupported,
    ApiVersionNotSupported,
    PlatformNotSupported,
    Other(&'static str),
}


trait Renderer: Send{
    fn render(&mut self,que: RenderQue);
}

pub struct Render{
    renderer: Box<Renderer>,
}

impl Render{
    pub fn new(window: WindowContext) -> Self{
        let renderer: Result<Box<Renderer>,Error> = match Registery::get("render.prefered").or("vulkan".to_string()).as_str(){
            "vulkan" => {
                Vulkan::new(window.clone()).map(|x| Box::new(x) as Box<Renderer>)
            },
            "opengl" => {
                Ogl::new(window.clone()).map(|x| Box::new(x) as Box<Renderer>)
            }
            x => {
                warn!("Renderer: {}, not supported, using vulkan.",x);
                Vulkan::new(window.clone()).map(|x| Box::new(x) as Box<Renderer>)
            }
        };
        let render = match renderer{
            Ok(x) => x,
            Err(e) =>{
                warn!("Could not initialize renderer: {:?}, falling back to opengl.",e);
                Box::new(Ogl::new(window).expect("Could not initialize fallback renderer"))
            }
        };
        Render{
            renderer: render,
        }
    }

    pub fn render(&mut self){
        let render = RenderQue{
            static_mesh: Vec::new(),
            camera: Camera{
                perspective: Perspective3::new(800.0/600.0,2.0,0.1,1000.0),
                rotation: UnitQuaternion::new(Vector3::new(0.0,0.0,1.0)),
                translation: Vector3::new(0.0,0.0,0.0),
            }
        };
        self.renderer.render(render);
    }
}
