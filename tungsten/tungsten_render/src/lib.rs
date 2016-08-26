#![crate_name = "tungsten_render"]
#![crate_type = "lib"]
#![allow(dead_code)]

extern crate task;
extern crate tungsten_core;
extern crate tungsten_asset;
#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;

use task::sync::mutate_inspect::{Inspector,Mutator};
use task::sync::mutate_inspect;

pub use tungsten_core::registery::Registery;
pub use tungsten_core::window::WindowContext;

mod format;
mod vulkan;
mod ogl;

pub use self::format::*;
use self::vulkan::Vulkan;
use self::ogl::Ogl;



#[derive(Debug)]
pub enum Error{
    ApiNotSupported,
    ApiVersionNotSupported,
    PlatformNotSupported,
    Other(&'static str),
}

type RenderObjects = Vec<Inspector<StaticRenderObject>>;
pub type RegisteredObject = Mutator<StaticRenderObject>;

/// Trait renderers must adhear to.
///
trait Renderer: Send{
    /// render the que given.
    fn render(&mut self,objects: &RenderObjects);
}


pub struct Render{
    renderer: Box<Renderer>,
    register_objects: Vec<Inspector<StaticRenderObject>>,
}

impl Render{
    pub fn new(window: WindowContext) -> Self{

        // find renderer in register
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

        // test if render could init.
        // If not try to fall back to ogl.
        let render = match renderer{
            Ok(x) => x,
            Err(e) =>{
                warn!("Could not initialize renderer: {:?}, falling back to opengl.",e);
                Box::new(Ogl::new(window).expect("Could not initialize fallback renderer"))
            }
        };

        Render{
            renderer: render,
            register_objects: Vec::new(),
        }
    }

    pub fn render(&mut self){
        self.renderer.render(&self.register_objects);
    }

    /// Register a render
    pub fn register(&mut self, object: StaticRenderObject) -> RegisteredObject{
        let (mutate,inspect) = mutate_inspect::mutate_inspect(object);
        println!("Registered!");
        self.register_objects.push(inspect);
        mutate
    }
}

