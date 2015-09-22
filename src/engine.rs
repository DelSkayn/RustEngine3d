use super::console;
use super::event;
use super::event::BaseEvent;
use super::event::KeyBoard;
use super::event::Key;
use super::window;

use std::rc::Rc;
pub struct Engine{
    console: Rc<console::Console>,
    window: Rc<window::Window>,
    event_loop: event::EventLoop<BaseEvent>,
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
        }
    }
    pub fn run(&mut self){
        'main : loop {
            self.event_loop.pull_events();
            for &event in self.event_loop.get_events(){
                println!("Events: {:?}",event);
                match event{
                    BaseEvent::Quit => break 'main,
                    BaseEvent::KeyBoard(KeyBoard::Pressed(Key::Esc)) => break 'main,
                    _ => (),
                }
            }
        }
    }

}

