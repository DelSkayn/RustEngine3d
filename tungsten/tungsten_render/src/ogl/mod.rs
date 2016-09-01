extern crate glium;

use self::glium::backend::{Context,Backend};
use self::glium::debug::{DebugCallbackBehavior,Severity};
use self::glium::{Frame,Surface};

use super::task::specific::Specific;
use super::task::ThreadId;

use super::tungsten_core::window::WindowContext;
use super::Error;
use super::Renderer;
use super::RenderObjects;

use std::rc::Rc;


mod pipeline;
use self::pipeline::static_mesh::PipeLine;
mod format;
pub use self::format::*;
mod cache;
use self::cache::Cache;

// Eh might cause problems with opengl context.
// It does cause problems need to stick it to a single thread.
unsafe impl Send for Ogl{}
unsafe impl Sync for Ogl{}

struct OglDefered{
    context: Rc<Context>,
    dimension: (u32,u32),
    cache: Cache,
    pipeline: PipeLine,
}

pub struct Ogl(OglDefered);

impl Ogl{
    pub fn new(window: WindowContext) -> Result<Self,Error>{
        let defer = Specific::new(|| OglDefered::new(window));
        defer.run(ThreadId::from_num(4));
        Ok(Ogl(try!(defer.get())))
    }
}

impl Renderer for Ogl{
    fn render(&mut self, que: &RenderObjects){
        let defer = Specific::new(|| self.0.render(que));
        defer.run(ThreadId::from_num(4));
    }
}


impl OglDefered{
    fn new(window: WindowContext) -> Result<Self,Error>{
        info!("Creating opengl renderer.");
        let dimensions = window.get_framebuffer_dimensions();
        let context = unsafe{
            Context::new::<WindowContext,()>(window,false,Self::get_debug_behavior()).unwrap()
        };

        let pipe = PipeLine::new(&context);

        let cache = Cache::new();

        Ok(OglDefered{
            context: context,
            dimension: dimensions,
            pipeline: pipe,
            cache: cache,
        })
    }

    fn get_debug_behavior() -> DebugCallbackBehavior{
        let callback = Box::new(|_,_,severity,_,error,message:&str|{
            let serv = match severity{
                Severity::Notification => "notification",
                Severity::Low => "low",
                Severity::Medium => "medium",
                Severity::High => "high",
            };
            if error{
                error!("[Opengl] {} #: {}",serv,message);
            }else{
                warn!("[Opengl] {} #: {}",serv,message);
            }

        });

        DebugCallbackBehavior::Custom{
            callback: callback,
            synchronous: false,
        }
    }

    fn render(&mut self, que: &RenderObjects){
        let mut frame = Frame::new(self.context.clone(),self.dimension);
        self.cache.load(&self.context,que);
        frame.clear_color(0.0,0.0,1.0,1.0);
        frame.clear_depth(1.0);
        self.pipeline.render(&self.cache,&mut frame);
        frame.finish().unwrap();
    }
}
