use super::console;
use super::event;
use super::event::BaseEvent;
use super::event::KeyBoard;
use super::event::Key;
use super::window;
use super::render::RenderEngine;
use super::render::RenderQueue;
use super::camera::Camera;
use super::Game;

use std::rc::Rc;
pub struct Engine<T: Game>{
    console: Rc<console::Console>,
    window: Rc<window::Window>,
    event_loop: event::EventLoop<BaseEvent>,
    renderengine: RenderEngine,
    running: bool,
    game: T,
}

impl<T: Game> Engine<T>{
    pub fn new() -> Self{
        println!("Engine starting!");
        trace!("Engine Startup.");
        let mut cons = console::Console::new();
        cons.add_command("quit",|_| Some(BaseEvent::Quit));

        let rc_cons = Rc::new(cons);

        let mut e_loop = event::EventLoop::<BaseEvent>::new();
        e_loop.register(rc_cons.clone());

        trace!("Window Creation.");
        let win = Rc::new(window::Window::new());
        e_loop.register(win.clone());
        trace!("Game setup.");

        trace!("Finising engine startup.");
        let renderengine = RenderEngine::new(win.clone());
        Engine{
            console: rc_cons,
            window: win,
            event_loop: e_loop,
            running: true,
            game: T::new(&renderengine),
            renderengine: renderengine,
        }
    }
    pub fn run(&mut self){
        trace!("Start running engine.");

        
        trace!("Start game loop.");
        while self.running{
            trace!("Start game itteration.");
            self.event_loop.pull_events();
            trace!("Start handeling events.");
            for &event in self.event_loop.get_events(){
                trace!("Events: {:?}",event);
                match event{
                    BaseEvent::Quit => {self.running = false;},
                    BaseEvent::KeyBoard(KeyBoard::Pressed(Key::Esc)) => {self.running = false;},
                    _ => (),
                }
            }

            self.game.update();

            let que = RenderQueue{
                queue: vec![],
                cam: Camera::new(),
            };

            

            trace!("End handeling events.");
                
            self.renderengine.render(self.game.render(que));
            trace!("End game itteration.");
        }
        info!("Quiting engine!");
    }

}

