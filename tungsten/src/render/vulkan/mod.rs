extern crate vulkano;

use self::vulkano::instance::{Instance,InstanceExtensions,PhysicalDevice,PhysicalDeviceType};

use std::sync::Arc;

pub struct Vulkan{
    instance: Arc<Instance>,
}

static LAYER_NAME: &'static str = "VK_LAYER_LUNARG_standard_validation";

impl Vulkan{
    pub fn new() -> Self{
        let extensions = InstanceExtensions::supported_by_core();
        let layers = vec![&LAYER_NAME];
        let instance = Instance::new(None,&extensions,layers).unwrap();

        {
            let mut device:Option<PhysicalDevice> = None;
            let itt = PhysicalDevice::enumerate(&instance);
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
                device = Some(dev);
            }
            if let None = device{
                error!("No rendering device found");
            }
            info!("Picked device: {}",device.as_ref().unwrap().name());
        }
        Vulkan{
            instance: instance,
        }
    }
}
