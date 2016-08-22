extern crate glium;

use self::glium::backend::{Context,Backend};
use self::glium::debug::{DebugCallbackBehavior,Severity};
use self::glium::{Frame,Surface,VertexBuffer,IndexBuffer};

use super::{Renderer,Error,WindowContext};
pub use super::RenderQue;

use std::collections::HashMap;
use std::rc::Rc;

mod format;
use self::format::*;

// Eh might cause problems with opengl context.
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
        frame.clear_color(0.0,0.0,self.temp,1.0);
        frame.finish().unwrap();
        self.temp += if self.dec {0.01} else {-0.01};
        if self.temp > 1.0{
            self.temp = 1.0;
            self.dec = false;
        }else if self.temp < 0.0{
            self.temp = 0.0;
            self.dec = true;
        }
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
