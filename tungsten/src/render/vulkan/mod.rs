extern crate vulkano;

use self::vulkano::instance::{Instance,InstanceExtensions,PhysicalDevice,PhysicalDeviceType};
use self::vulkano::device::{Device,DeviceExtensions};

use std::sync::Arc;

pub struct Vulkan{
    instance: Arc<Instance>,
    device: Arc<Device>,
}

static LAYER_NAME: &'static str = "VK_LAYER_LUNARG_standard_validation";

#[derive(Eq,PartialEq,Ord,PartialOrd,Clone,Copy)]
struct DeviceRating(usize);

impl DeviceRating{
    fn lowest() -> DeviceRating{
        DeviceRating(0)
    }
}

impl Vulkan{
    pub fn new() -> Self{
        let extensions = InstanceExtensions::supported_by_core();
        let layers = vec![&LAYER_NAME];
        let instance = Instance::new(None,&extensions,layers).unwrap();
        let device = Self::pick_device(&instance);
        Vulkan{
            instance: instance,
            device: device,
        }
    }

    fn pick_device(instance: &Arc<Instance>) -> Arc<Device>{
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
        let features = device.supported_features();
        let dev_extension = DeviceExtensions::none();
        info!("Picked device: {}",device.name());
        // TODO remove unwrap
        let (a,_) = Device::new(&device,&features,&dev_extension,Vec::new(),Vec::new()).unwrap();
        a
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


