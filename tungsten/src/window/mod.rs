extern crate glutin;

use registry::Registry;

use self::glutin::Window as WindowExt;
use self::glutin::Event as EventExt;
use self::glutin::ElementState;
use self::glutin::VirtualKeyCode;
use self::glutin::WindowBuilder as WindowExtBuilder;

pub struct Window{
    window: WindowExt,
    quit_on_esc: bool,
}

impl Window{
    pub fn from_registry() -> Self{
        info!("Creating window from settings!");
        let dimensions: [u64; 2] = Registry::get("window.size").unwrap();
        let vsync = Registry::get("window.vsync").unwrap();
        let position: [u64; 2] = Registry::get("window.position").unwrap();
        let title = Registry::get("window.title").unwrap();
        let quit_on_esc = Registry::get("general.quit_on_esc")
            .unwrap();
        info!("Window size: {}x{}",dimensions[0],dimensions[1]);
        info!("Window position: {}x{}",position[0],position[1]);
        info!("vsync: {}",vsync);
        let mut window_builder = WindowExtBuilder::new()
            .with_dimensions(dimensions[0] as u32,dimensions[1] as u32)
            .with_title(title);
        if vsync {
            window_builder = window_builder.with_vsync();
        }
        let window = window_builder.build().unwrap();
        window.set_position(position[0] as i32,position[1] as i32);
        Window{
            window: window,
            quit_on_esc: quit_on_esc,
        }
    }

    pub fn new() -> Self{
        Window{
            window: WindowExt::new().unwrap(),
            quit_on_esc: false,
        }
    }

    pub fn update(&mut self){
        self.window.swap_buffers().unwrap();
        for event in self.window.poll_events(){
            match event{
                EventExt::Closed => {
                    Registry::quit();
                },
                EventExt::KeyboardInput(ElementState::Pressed,_
                                        ,Some(VirtualKeyCode::Escape)) =>
                {
                    if self.quit_on_esc{
                        Registry::quit();
                    }
                },
                _ => {}
            }
        }
    }
}
