use registery::Registery;
use state::State;
use window::Window;
use util::Logger;
use console::{Console, SystemTerminal};
use render::Render;
use Game;
use task;


const BANNER: &'static str = r#"
  ______                                        __                       
 /\__  _\                                      /\ \__                    
 \/_/\ \/   __  __    ___       __       ____  \ \ ,_\     __     ___    
    \ \ \  /\ \/\ \ /' _ `\   /'_ `\    /',__\  \ \ \/   /'__`\ /' _ `\  
     \ \ \ \ \ \_\ \/\ \/\ \ /\ \L\ \  /\__, `\  \ \ \_ /\  __/ /\ \/\ \ 
      \ \_\ \ \____/\ \_\ \_\\ \____ \ \/\____/   \ \__\\ \____\\ \_\ \_\
       \/_/  \/___/  \/_/\/_/ \/___L\ \ \/___/     \/__/ \/____/ \/_/\/_/
                                /\____/                                  
                                \_/__/                                   
"#;

/// The engine object holds all the data neccesary for the engine.
pub struct Engine<G: Game + Send> {
    game: G,
    window: Window,
    console: Console<SystemTerminal>,
    render: Render,
}

impl<G: Game + Send> Engine<G> {

    /// Run the engine.
    #[allow(non_snake_case)]
    pub fn Go() {
        println!("\x1B[1;31m--------------------------------------------------------------------------\x1B[0m");
        println!("\x1B[1;31m{}\x1B[0m", BANNER);
        println!("\x1B[1;31m--------------------------- Engine Starting! -----------------------------\x1B[0m");


        Logger::init().unwrap();
        Registery::read_from_file();
        let console = Console::new(SystemTerminal::new());
        let window = Window::from_registry();
        let render = Render::new(window.get_context());
        Engine {
                game: G::new(),
                window: window,
                console: console,
                render: render,
            }
            .game_loop();
        println!("\x1B[1;31m---------------------------- Engine Quit! --------------------------------\x1B[0m");
    }

    fn game_loop(&mut self) {
        while State::running(){
            let window = &mut self.window;
            let console = &mut self.console;
            task::join(|| window.update(), || console.update());
            self.render.render();
        }
    }
}
