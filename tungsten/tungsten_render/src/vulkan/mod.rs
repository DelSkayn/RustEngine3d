extern crate vulkano;

mod swapchain;
mod device;

use self::swapchain::Swapchain;
use self::device::Device;

use self::vulkano::instance::{Instance,InstanceExtensions};
use self::vulkano::swapchain::Surface;
use self::vulkano::instance::debug::{DebugCallback,MessageTypes};

use std::sync::Arc;

use super::tungsten_core::registery::Registery;

pub use super::{Error,WindowContext,RenderQue};
use super::{Renderer,RenderObjects};

unsafe impl Send for Vulkan{}

pub struct Vulkan{
    instance: Arc<Instance>,
    device: Device,
    swapchain: Swapchain,
    surface: Arc<Surface>,
    log: DebugCallback,
}

static LAYER_NAME_1: &'static str = "VK_LAYER_LUNARG_standard_validation";
static LAYER_NAME_2: &'static str = "VK_LAYER_LUNARG_image";
static LAYER_NAME_3: &'static str = "VK_LAYER_LUNARG_parameter_validation";
static LAYER_NAME_4: &'static str = "VK_LAYER_LUNARG_core_validation";
static LAYER_NAME_5: &'static str = "VK_LAYER_LUNARG_object_tracker";
static LAYER_NAME_6: &'static str = "VK_LAYER_LUNARG_swapchain";

impl Renderer for Vulkan{
    fn render(&mut self,_: &RenderObjects){
    }
}

impl Vulkan{

    pub fn new(window: WindowContext) -> Result<Self,Error>{
        info!("Creating vulkan renderer.");
        let extensions = InstanceExtensions::supported_by_core();
        let layers = vec![
            &LAYER_NAME_1,
            &LAYER_NAME_2,
            &LAYER_NAME_3,
            &LAYER_NAME_4,
            &LAYER_NAME_5,
            &LAYER_NAME_6,
        ];
        let instance = Instance::new(None,&extensions,layers).unwrap();
        let log = Self::create_debug_logger(&instance);
        let surface = try!(Self::get_surface(&instance,window.clone()));
        let device = try!(Device::new(&instance,&surface));
        let swapchain = try!(Swapchain::new(&device,&surface,window));
        Ok(Vulkan{
            instance: instance,
            device: device,
            swapchain: swapchain,
            surface: surface,
            log: log,
        })
    }

    fn create_debug_logger(instance: &Arc<Instance>) -> DebugCallback{
        info!("Creating debug logger for vulkan.");
        let warning = Registery::get("render.vulkan.log.warning").or(true);
        let ty = MessageTypes{
            debug: Registery::get("render.vulkan.log.debug").or(false),
            warning: warning,
            performance_warning: warning,
            information: Registery::get("render.vulkan.log.information").or(false),
            error: Registery::get("render.vulkan.log.error").or(true),
        };
        DebugCallback::new(instance,ty,|ref message|{
            if message.ty.error{
                error!("[VULKAN] layer: {} #:{}",message.layer_prefix,message.description);
            }else if message.ty.warning || message.ty.performance_warning{
                warn!("[VULKAN] layer: {} #:{}",message.layer_prefix,message.description);
            }else if message.ty.information{
                info!("[VULKAN] layer: {} #:{}",message.layer_prefix,message.description);
            }else if message.ty.debug{
                debug!("[VULKAN] layer: {} #:{}",message.layer_prefix,message.description);
            }
        }).unwrap()
    }

    #[cfg(unix)]
    fn get_surface(instance: &Arc<Instance>,window: WindowContext) -> Result<Arc<Surface>,Error>{
        unsafe{
            let display = try!(window.get_display_ptr().ok_or(Error::PlatformNotSupported));
            Surface::from_xlib(instance,display,window.get_window_ptr().unwrap())
                .map_err(|_| Error::Other("Could not create surface"))

        }
    }

    #[cfg(not(unix))]
    fn get_surface(instance: &Arc<Instance>) -> Result<Surface,Error>{
        unimplemented!();
    }
}
