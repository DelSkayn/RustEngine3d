extern crate glutin;
extern crate glium;
extern crate libc;

use super::registery::Registery;
use super::state::State;

use self::glium::backend::Backend;
use self::glium::SwapBuffersError;

use self::libc::c_void;

use self::glutin::Window as GliumWindow;
use self::glutin::Event as EventExt;
use self::glutin::ElementState;
use self::glutin::VirtualKeyCode;
use self::glutin::WindowBuilder as GliumWindowBuilder;
use self::glutin::ContextError;

use std::os::raw::c_void as c_void_std;
use std::sync::Arc;
use std::mem;

#[cfg(macos)]
use self::glutin::os::macos::WindowExt;
#[cfg(unix)]
use window::glutin::os::unix::WindowExt;
#[cfg(windows)]
use self::glutin::os::windows::WindowExt;



#[derive(Clone)]
pub struct WindowContext{
    r: Arc<GliumWindow>,
}

unsafe impl Backend for WindowContext{
    fn swap_buffers(&self) -> Result<(),SwapBuffersError>{
        self.r.swap_buffers().map_err(|e|{
            match e {
                ContextError::IoError(_) => panic!(),
                ContextError::ContextLost => SwapBuffersError::ContextLost,
            }
        })
    }

    unsafe fn get_proc_address(&self,symbol: &str) -> *const c_void_std{
        mem::transmute(self.r.get_proc_address(symbol))
    }

    fn get_framebuffer_dimensions(&self) -> (u32,u32){
        self.r.get_inner_size_pixels().unwrap()
    }

    fn is_current(&self) -> bool{
        self.r.is_current()
    }

    unsafe fn make_current(&self){
        self.r.make_current().unwrap()
    }
}

#[cfg(unix)]
impl WindowContext{
    pub fn get_window_ptr(&self) -> Option<*mut c_void>{
        unsafe{
            mem::transmute(self.r.get_xlib_window())
        }
    }

    pub fn get_display_ptr(&self) -> Option<*mut c_void>{
        unsafe{
            mem::transmute(self.r.get_xlib_display())
        }
    }
}

#[cfg(macos)]
impl WindowContext{
    fn get_window_ptr(&self) -> Option<*mut c_void>{
        Some(mem::transmute(self.get_nswindow()))
    }

    pub fn get_display_ptr(&self) -> Option<*mut c_void>{
        panic!("Function not supported in macos");
    }
}

#[cfg(windows)]
impl WindowContext{
    fn get_window_ptr(&self) -> Option<*mut c_void>{
        Some(mem::transmute(self.get_hwnd()))
    }

    pub fn get_display_ptr(&self) -> Option<*mut c_void>{
        panic!("Function not supported in windows");
    }
}

pub struct Window {
    window: Arc<GliumWindow>,
    quit_on_esc: bool,
}

impl Window {
    pub fn from_registry() -> Self {
        info!("Creating window from settings!");

        let dimensions: [u64; 2] = Registery::get("window.size").or([300,300]);
        let vsync = Registery::get("window.vsync").or(false);
        let position: [u64; 2] = Registery::get("window.position").or([0,0]);
        let title = Registery::get("window.title").or("Tungsten engine".to_string());
        let quit_on_esc = Registery::get("general.quit_on_esc").or(true);

        info!("Window size: {}x{}", dimensions[0], dimensions[1]);
        info!("Window position: {}x{}", position[0], position[1]);
        info!("vsync: {}", vsync);
        let window_builder = GliumWindowBuilder::new()
            .with_dimensions(dimensions[0] as u32, dimensions[1] as u32)
            .with_title(title);
        let window = window_builder.build().unwrap();
        window.set_position(position[0] as i32, position[1] as i32);
        Window {
            window: Arc::new(window),
            quit_on_esc: quit_on_esc,
        }
    }

    pub fn new() -> Self {
        Window {
            window: Arc::new(GliumWindow::new().unwrap()),
            quit_on_esc: false,
        }
    }

    pub fn update(&mut self) {
        //self.window.swap_buffers().unwrap();
        for event in self.window.poll_events() {
            match event {
                EventExt::Closed => {
                    State::quit();
                }
                EventExt::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                    if self.quit_on_esc {
                        State::quit();
                    }
                }
                _ => {}
            }
        }
    }

    pub fn get_context(&self) -> WindowContext{
        WindowContext{
            r: self.window.clone(),
        }
    }
}
