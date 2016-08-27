
use super::vulkano::instance::{Instance,PhysicalDevice,PhysicalDeviceType};
use super::vulkano::device::{Device as DeviceExt,DeviceExtensions,Queue};
use super::vulkano::swapchain::Surface;

use super::super::Error;

use std::sync::Arc;

#[derive(Eq,PartialEq,Ord,PartialOrd,Clone,Copy)]
pub struct DeviceRating(usize);

impl DeviceRating{
    pub fn lowest() -> DeviceRating{
        DeviceRating(0)
    }
}

pub struct Device{
    device: Arc<DeviceExt>,
    queue: Vec<Arc<Queue>>,
    rating: DeviceRating,
}

impl Device{
    pub fn new(instance: &Arc<Instance>,surface: &Arc<Surface>) -> Result<Self,Error>{
        let mut device: Option<PhysicalDevice> = None;
        let mut dev_rating = DeviceRating::lowest();

        // Find proper physical device.
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
            //error!("No rendering device found.");
            return Err(Error::Other("Could not find vulkan capable device"));
        }

        let device = device.unwrap();
        // TODO: Test if features are supported.
        let features = device.supported_features();
        let dev_extension = DeviceExtensions{
            khr_swapchain: true,
            khr_display_swapchain: false,
        };

        let que = try!(device.queue_families().find(|q|{
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        }).ok_or(Error::Other("Could not find supporting queue")));

        info!("Picked device: {}",device.name());
        let (dev,que) = try!(DeviceExt::new(&device,&features,&dev_extension,[(que,0.5)].iter().cloned())
                        .map_err(|_| Error::Other("Could not create device")));
        Ok(Device{
            device: dev,
            queue: que.collect(),
            rating: dev_rating,
        })
    }

    pub fn get_rating(&self) -> DeviceRating{
        self.rating
    }

    pub fn get_queue(&self) -> &[Arc<Queue>]{
        &self.queue
    }

    pub fn get_inner(&self) -> &Arc<DeviceExt>{
        &self.device
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

