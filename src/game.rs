use super::Event;
use super::CoreEvent;

use super::render::RenderEvent;

use std::thread;

use super::time;
use std::time::Duration;

static FRAME_TIME_NS:u64 = 16666;

use super::kernal::{
    System,
    EventHandle,
};

pub trait Game{
    fn new() -> Self;
    fn render(&mut self);
    fn update(&mut self);
}


struct GameSystem<T: Game>{
    game: T,
    event: EventHandle,
    delta: f64,
    time_frame: u64,
    frame_id: u64,
}

impl<T: Game> GameSystem<T>{
    fn new(game: T,event: EventHandle) -> Self{
        GameSystem{
            game: game,
            event: event,
            delta: 0.0,
            time_frame: 0,
            frame_id: 0,
        }
    }
}

impl<T: Game> System for GameSystem<T>{
    fn run(&mut self){
        for e in self.event.into_iter(){
            match e {
                Event::Render(RenderEvent::FrameDone) => {
                    self.event.push(Event::Core(CoreEvent::FrameDone(self.frame_id)));
                    let time = time::precise_time_ns();
                    if time - self.time_frame < FRAME_TIME_NS{
                        thread::sleep(Duration::new(0,(FRAME_TIME_NS - (time - self.time_frame)) as u32));
                    }
                    self.frame_id += 1;
                    self.event.push(Event::Core(CoreEvent::Frame(self.frame_id)));
                }
                _ => {},
            }
        }
    }
}

