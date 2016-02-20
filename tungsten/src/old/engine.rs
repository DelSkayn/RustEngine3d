use super::console::Console;
use super::game::Game;
use super::Event;
use super::CoreEvent;
use super::window::Window;

use super::render::RenderSystem;
use super::render::basic::BasicRenderer;

use super::time;
use super::profile::ProfileSample;

use super::kernal::{
    System,
    KernalBuilder,
    Kernal
};


pub struct Engine{
    kernal: Kernal,
}

impl Engine{
    pub fn new<T: Game>() -> Self{
        println!("## Engine version: {}.{}.{} starting! ##\n"
                 ,super::VERSION_MAJOR,super::VERSION_MINOR
                 ,super::VERSION_PATCH);
        let mut builder = KernalBuilder::new();

        let mut console = Box::new(Console::new(builder.get_event_handle()));
        trace!("Engine Startup.");

        console.add_command("quit",|_| {
            Some(Event::Core(CoreEvent::Quit))
        });

        console.add_command("profile_events",|_| {
            Some(Event::Profile(time::precise_time_s()))
        });

        console.add_command("print_profile",|_| {
            ProfileSample::print();
            None
        });

        let window = Box::new(Window::new(builder.get_event_handle()));

        let render_engine = Box::new(RenderSystem::<BasicRenderer>::new(
            window.get_display().clone()
            ,builder.get_event_handle()));

        builder.add_system(console);
        builder.add_system(window);
        builder.add_system(render_engine);

        let kernal = builder.build();

        Engine{
            kernal: kernal,
        }
    }
    pub fn run(&mut self){
        trace!("Starting engine.");
        self.kernal.run();
        info!("Quiting engine!");
    }

}


