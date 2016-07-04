mod vulkan;

use self::vulkan::Vulkan;

#[derive(Debug)]
enum Error{
    ApiNotSupported,
    ApiVersionNotSupported,
    Other(&'static str),
}

trait Renderer: Sized{
    fn new() -> Result<Self,Error>;
}

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
