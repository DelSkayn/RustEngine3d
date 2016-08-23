extern crate glium;

use self::glium::backend::{Context,Backend};
use self::glium::debug::{DebugCallbackBehavior,Severity};
use self::glium::{Frame,Surface};

use super::{Renderer,Error,WindowContext};
pub use super::RenderQue;

use std::rc::Rc;

mod format;
pub use self::format::*;

// Eh might cause problems with opengl context.
// It does cause problems need to stick it to a single thread.
unsafe impl Send for Ogl{}

pub struct Ogl{
    context: Rc<Context>,
    dimension: (u32,u32),
    temp: f32,
    dec: bool
}

impl Renderer for Ogl{
    fn render(&mut self, que: RenderQue){
        let mut frame = Frame::new(self.context.clone(),self.dimension);
        frame.clear_color(0.0,0.0,1.0,1.0);
        frame.finish().unwrap();
    }
}

impl Ogl{
    pub fn new(window: WindowContext) -> Result<Self,Error>{
        info!("Creating opengl renderer.");
        let dimensions = window.get_framebuffer_dimensions();
        let context = unsafe{
            Context::new::<WindowContext,()>(window,false,Self::get_debug_behavior()).unwrap()
        };

        Ok(Ogl{
            context: context,
            dimension: dimensions,
            temp: 0.0,
            dec: true,
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
}
