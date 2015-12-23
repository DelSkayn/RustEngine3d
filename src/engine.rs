use super::console::Console;
use super::window;
use super::render::BasicRenderer;
use super::render::RenderEngine;
use super::render::RenderQueue;
use super::render::camera::Camera;
use super::Game;
use super::thread_pool::ThreadPool;
use super::Event;

use super::kernal::{
    KernalBuilder,
    Kernal
};

use std::rc::Rc;

pub struct Engine<T: Game>{
    kernal: Kernal,
    game: T,
}

impl<T: Game> Engine<T>{
    pub fn new() -> Self{
        println!("## Engine version: {}.{}.{} starting! ##\n"
                 ,super::VERSION_MAJOR,super::VERSION_MINOR
                 ,super::VERSION_PATCH);
        trace!("Engine Startup.");
        let mut builder = KernalBuilder::new();

        let console = Console::new(builder.get_event_handle());

        let kernal = builder.build();

        Engine{
            kernal: kernal,
            game: T::new(),
        }
    }
    pub fn run(&mut self){
        trace!("Start running engine.");
        self.kernal.run();
        info!("Quiting engine!");
    }

}

