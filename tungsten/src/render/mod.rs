mod vulkan;

use self::vulkan::Vulkan;

pub struct Render{
    renderer: Vulkan,
}

impl Render{
    pub fn new() -> Self{
        Render{
            renderer: Vulkan::new(),
        }
    }
}
