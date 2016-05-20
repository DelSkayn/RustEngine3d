extern crate glutin;

use settings::Settings;

use self::glutin::Window as WindowExt;
use self::glutin::Event as EventExt;
use self::glutin::WindowBuilder as WindowExtBuilder;

pub struct Window{
    window: WindowExt,
}

impl Window{
    pub fn from_settings() -> Self{
        info!("Creating window from settings!");
        let dimensions: [u64; 2] = Settings::get("window_size");
        let vsync = Settings::get("vsync");
        let position: [u64; 2] = Settings::get("window_pos");
        let title = Settings::get("window_title");
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
        }
    }

    pub fn new() -> Self{
        Window{
            window: WindowExt::new().unwrap(),
        }
    }

    pub fn update(&mut self){
        for event in self.window.poll_events(){
            match event{
                EventExt::Closed => {
                    Settings::quit();
                },
                _ => {}
            }
        }
    }
}
