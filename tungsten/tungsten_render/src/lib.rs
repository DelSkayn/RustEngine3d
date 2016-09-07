//! This crate profides the rendering functionality of the tungsten game engine. The render crate
//! profides functions for rendering on both opengl and vulkan. Where opengl is used as a fallback
//! for vulkan.
//!
//! # A summary
//!
//! - The `Render` struct is the root struct and contains all the data the rendering engine needs
//!   to render.
//!
//! - In order to render the user needs to register a render object at the render engine. This
//!   object can then be mutated as nessacary and will be read from by the render engine when it
//!   needs the data.
//!

#![crate_name = "tungsten_render"]
#![crate_type = "lib"]
#![allow(dead_code)]

#[macro_use]
extern crate log;
extern crate task;
extern crate tungsten_core;
extern crate tungsten_asset;
extern crate tungsten_render_ogl;
extern crate tungsten_render_vulkan;

use task::sync::mutate_inspect::{Inspector,Mutator};
use task::sync::mutate_inspect;

use tungsten_core::registery::Registery;
use tungsten_core::window::WindowContext;
use tungsten_asset::{AssetData,Mesh};

mod format;

pub use self::format::*;

#[derive(Debug)]
pub enum Error{
    ApiNotSupported,
    ApiVersionNotSupported,
    PlatformNotSupported,
    Other(&'static str),
}

type RenderObjects = Vec<RegisterData>;
pub type RegisteredObject = Mutator<StaticRenderObject>;

/// Trait renderers must adhear to.
///
trait Renderer: Send{
    /// render the que given.
    fn render(&mut self,objects: &RenderObjects);
}

pub struct RegisterData{
    object: Inspector<StaticRenderObject>,
    mesh: AssetData<Mesh>
}

impl RegisterData{
    pub fn object(&self) -> &Inspector<StaticRenderObject>{
        &self.object
    }

    pub fn mesh(&self) -> &AssetData<Mesh>{
        &self.mesh
    }
}



pub struct Render{
    renderer: Box<Renderer>,
    objects: Vec<RegisterData>,
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
            objects: Vec::new(),
        }
    }

    pub fn render(&mut self){
        self.renderer.render(&self.objects);
    }

    /// Register a render
    pub fn register(&mut self, object: StaticRenderObject,mesh: AssetData<Mesh>) -> RegisteredObject{
        let (mutate,inspect) = mutate_inspect::mutate_inspect(object);
        self.objects.push(RegisterData{ mesh: mesh,object: inspect});
        mutate
    }
}

