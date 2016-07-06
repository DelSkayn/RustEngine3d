extern crate vulkano;

use self::vulkano::instance::{Instance,InstanceExtensions,Features,PhysicalDevice,PhysicalDeviceType};
use self::vulkano::instance::debug::{DebugCallback,MessageTypes};
use self::vulkano::device::{Device,DeviceExtensions};

use std::sync::Arc;

use registery::Registery;

use super::{Renderer,Error};

pub struct Vulkan{
    instance: Arc<Instance>,
    device: Arc<Device>,
    log: DebugCallback,
}

static LAYER_NAME: &'static str = "VK_LAYER_LUNARG_standard_validation";

#[derive(Eq,PartialEq,Ord,PartialOrd,Clone,Copy)]
struct DeviceRating(usize);

impl DeviceRating{
    fn lowest() -> DeviceRating{
        DeviceRating(0)
    }
}

impl Renderer for Vulkan{}

impl Vulkan{
    pub fn new() -> Result<Self,Error>{
        info!("Creating vulkan renderer.");
        let extensions = InstanceExtensions::supported_by_core();
        let layers = vec![&LAYER_NAME];
        let instance = Instance::new(None,&extensions,layers).unwrap();
        let device = match Self::create_device(&instance){
            Some(x) => x,
            None => return Err(Error::Other("Could not create device")),
        };
        let log = Self::create_debug_logger(&instance);
        Ok(Vulkan{
            instance: instance,
            device: device,
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
                error!("[VULKAN] layer: {} \n   {}",message.layer_prefix,message.description);
            }else if message.ty.warning || message.ty.performance_warning{
                warn!("[VULKAN] layer: {} \n   {}",message.layer_prefix,message.description);
            }else if message.ty.information{
                info!("[VULKAN] layer: {} \n   {}",message.layer_prefix,message.description);
            }else if message.ty.debug{
                debug!("[VULKAN] layer: {} \n   {}",message.layer_prefix,message.description);
            }
        }).unwrap()
    }

    fn create_device(instance: &Arc<Instance>) -> Option<Arc<Device>>{
        let mut device:Option<PhysicalDevice> = None;
        let mut dev_rating = DeviceRating::lowest();

        let itt = PhysicalDevice::enumerate(instance);
        for dev in itt{
            info!("Found Physical Rendering device.");
            info!("name: {}",dev.name());
            info!("vulkan api version: {:?}", dev.api_version());
            let type_name = match dev.ty(){
                PhysicalDeviceType::IntegratedGpu => "Intergrated",
                PhysicalDeviceType::DiscreteGpu => "Discrete",
                PhysicalDeviceType::VirtualGpu => "Virtual",
                PhysicalDeviceType::Cpu => "Cpu",
                PhysicalDeviceType::Other=> "Type not regocnized",
            };
            info!("device type: {}",type_name);
            if device.is_none(){
                device = Some(dev);
            }else if dev_rating < Self::get_device_rating(&dev){
                dev_rating = Self::get_device_rating(&dev);
                device = Some(dev);
            }
        }
        if let None = device{
            error!("No rendering device found");
        }
        let device = device.unwrap();
        // TODO: Test if features are supported.
        let features = Features::none();
        let dev_extension = DeviceExtensions::none();
        info!("Picked device: {}",device.name());
        // TODO remove unwrap
        let (a,_) = match Device::new(&device,&features
                                      ,&dev_extension
                                      ,Vec::new()
                                      ,Vec::new())
        {
            Ok(x) => x,
            Err(_) => return None,
        };
        Some(a)
    }

    fn get_device_rating(device: &PhysicalDevice) -> DeviceRating{
        // TODO: Extend rating device.
        DeviceRating(match device.ty(){
            PhysicalDeviceType::IntegratedGpu => 2,
            PhysicalDeviceType::DiscreteGpu => 3,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 1, 
            PhysicalDeviceType::Other=> 0,
        })
    }
}
