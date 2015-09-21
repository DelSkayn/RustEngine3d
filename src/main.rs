extern crate engine3d;
use engine3d::console;
use engine3d::event;
use engine3d::event::BaseEvent;
use engine3d::window;

use std::cell::RefCell;
use std::rc::Rc;

fn main(){
    let cons = Rc::new(RefCell::new(console::Console::new()));
    cons.borrow_mut().add_command("quit",|args| Some(BaseEvent::Quit));

    let mut e_loop = event::EventLoop::<BaseEvent>::new();
    e_loop.register(cons.clone());

    let win = Rc::new(RefCell::new(window::Window::new()));
    e_loop.register(win.clone());

    'main : loop {
        e_loop.pull_events();
        for &event in e_loop.get_events(){
            println!("Events: {:?}",event);
            match event{
                BaseEvent::Quit => break 'main,
                _ => (),
            }
        }
    }

}
