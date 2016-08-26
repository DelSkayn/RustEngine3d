
use super::vulkano::swapchain::{Swapchain as SwapchainExt,Surface,SurfaceTransform};
use super::vulkano::image::swapchain::SwapchainImage;

use std::sync::Arc;

use super::{WindowContext,Error};
use super::device::Device;

pub struct Swapchain{
    images: Vec<Arc<SwapchainImage>>,
    swapchain: Arc<SwapchainExt>,
    window: WindowContext,
}

impl Swapchain{
    pub fn new(device: &Device
           ,surface: &Arc<Surface>
           ,window: WindowContext) -> Result<Self,Error>{

        let cap = try!(surface.get_capabilities(&device.get_inner().physical_device())
                       .map_err(|_| Error::Other("Could not get capabilities")));
        let que: Vec<&_> = device.get_queue().iter().collect();
        let dimensions = cap.current_extent.unwrap_or([1280,1024]);
        let present = cap.present_modes.iter().next().unwrap().clone();
        let alpha = cap.supported_composite_alpha.iter().next().unwrap().clone();
        let format = cap.supported_formats[0].0;
        let (swapchain,images) = try!(SwapchainExt::new(device.get_inner()
                                       ,&surface ,2
                                       ,format ,dimensions
                                       ,1 ,&cap.supported_usage_flags
                                       ,que.as_slice() ,SurfaceTransform::Identity
                                       ,alpha ,present
                                       ,true ,None).map_err(|_| Error::Other("Could not create swapchain.")));
        Ok(Swapchain{
            swapchain: swapchain,
            images: images,
            window: window,
        })
    }

}
