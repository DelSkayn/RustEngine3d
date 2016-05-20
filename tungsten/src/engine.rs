use settings::Settings;

use super::Game;
use super::window::Window;
use super::util::Logger;

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

pub struct Engine<G: Game>{
    game: G,
    window: Window,
}

impl<G: Game> Engine<G>{
    #[allow(non_snake_case)]
    pub fn Go(){
        println!("--------------------------------------------------------------------------");
        println!("{}",BANNER);
        println!("--------------------------- Engine Starting! -----------------------------");
    
        let _game = G::new();

        Logger::init().unwrap();
        Settings::read_from_file();
        Engine{
            game: G::new(),
            window: Window::from_settings(),
        }.game_loop();
        println!("---------------------------- Engine Quit! --------------------------------");
    }

    fn game_loop(&mut self){
        loop{
            self.window.update();
            if !Settings::running(){
                break;
            }
        }
    }
}
