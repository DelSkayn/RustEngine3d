extern crate task;
use task::sync::mutate_inspect::{self,Inspector,Mutator};

mod format;
pub use self::format::*;

mod vulkan;
use self::vulkan::Vulkan;
mod ogl;
use self::ogl::Ogl;

pub use registery::Registery;
pub use window::WindowContext;


#[derive(Debug)]
pub enum Error{
    ApiNotSupported,
    ApiVersionNotSupported,
    PlatformNotSupported,
    Other(&'static str),
}

type RenderObjects = Vec<Inspector<StaticRenderObject>>;

/// Trait renderers must adhear to.
///
trait Renderer: Send{
    /// render the que given.
    fn render(&mut self,objects: RenderObjects);

    // Register a render object in the renderer.
    // Possibly loading the mesh and caching data.
    //fn register(&self,RenderObjectHandle);
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

    pub fn render(&self){
        unimplemented!();
    }

    /// Register a render
    pub fn register(&mut self, object: StaticRenderObject) -> Mutator<StaticRenderObject>{
        let (mutate,inspect) = mutate_inspect::mutate_inspect(object);
        self.register_objects.push(inspect);
        mutate
    }
}

