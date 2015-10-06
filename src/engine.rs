use super::console;
use super::event;
use super::event::BaseEvent;
use super::event::KeyBoard;
use super::event::Key;
use super::window;
use super::render::RenderEngine;
use super::render::RenderObject;
use super::render::RenderQueue;
use super::obj::ObjLoader; 
use super::math::Matrix4f;
use super::math::Vector3f;
use super::camera::Camera;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use std::rc::Rc;
pub struct Engine{
    console: Rc<console::Console>,
    window: Rc<window::Window>,
    event_loop: event::EventLoop<BaseEvent>,
    running: bool,
}

impl Engine{
    pub fn new() -> Self{
        let mut cons = console::Console::new();
        cons.add_command("quit",|_| Some(BaseEvent::Quit));

        let rc_cons = Rc::new(cons);

        let mut e_loop = event::EventLoop::<BaseEvent>::new();
        e_loop.register(rc_cons.clone());

        let win = Rc::new(window::Window::new());
        e_loop.register(win.clone());
        Engine{
            console: rc_cons,
            window: win,
            event_loop: e_loop,
            running: true,
        }
    }
    pub fn run(&mut self){
        let renderengine = RenderEngine::new(self.window.clone());

        let mut src = String::new();
        BufReader::new(
            File::open("res/teapot.obj").unwrap()
        ).read_to_string(&mut src).unwrap();
        let mesh = renderengine.create_mesh(
            &ObjLoader::new(src).load().unwrap()
            ).unwrap();

        let ren_obj = RenderObject{
            mesh: &mesh,
            transform: Matrix4f::as_translation(Vector3f::from_coords(1.0,1.0,0.0)),
        };
        
        let que = RenderQueue{
            queue: vec![ren_obj],
            cam: Camera::with_perspective(90.0,800.0/600.0,10.0,0.001),
        };

        while self.running{
            self.event_loop.pull_events();
            for &event in self.event_loop.get_events(){
                debug!("Events: {:?}",event);
                match event{
                    BaseEvent::Quit => {self.running = false;},
                    BaseEvent::KeyBoard(KeyBoard::Pressed(Key::Esc)) => {self.running = false;},
                    _ => (),
                }
            }
                
            renderengine.render(que.clone());
        }
    }

}

