use registry::Registry;

use Game;

use task;

use window::Window;
use util::Logger;

use console::{Console, SystemTerminal};

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
}

impl<G: Game + Send> Engine<G> {

    /// Run the engine.
    #[allow(non_snake_case)]
    pub fn Go() {
        println!("--------------------------------------------------------------------------");
        println!("{}", BANNER);
        println!("--------------------------- Engine Starting! -----------------------------");


        Logger::init().unwrap();
        Registry::read_from_file();
        let console = Console::new(SystemTerminal::new());
        Engine {
                game: G::new(),
                window: Window::from_registry(),
                console: console,
            }
            .game_loop();
        println!("---------------------------- Engine Quit! --------------------------------");
    }

    fn game_loop(&mut self) {
        while Registry::running() {
            let window = &mut self.window;
            let console = &mut self.console;
            task::join(|| window.update(), || console.update());
        }
    }
}
