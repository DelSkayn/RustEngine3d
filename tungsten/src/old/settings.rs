//! 
//! This module handle game engine options. 
//! For now it is mostly a struct containing some data, 
//! but eventually it will also deal with loading a config file and managing runtime changes.
//!
//!

use super::serde_json;

use super::io;
use super::io::IOError;

use std::sync::RwLock;

use std::convert::AsRef;

use std::default::Default;

use std::path::Path;

use std::ops::Deref;

lazy_static!{
    static ref SETTINGS: RwLock<SettingsInner> = {
        RwLock::new(SettingsInner::default())
    };
}

///
/// A struct containing all the settings in the engine divided in catagories as 
/// seen in option menus.
///
pub struct Settings;

#[derive(Serialize, Deserialize, Default)]
pub struct SettingsInner{
    /// Options for the graphic part of the engine.
    pub graphics: Graphics,
}

/// struct containing options for the graphic part of the engine.
#[derive(Serialize,Deserialize)]
pub struct Graphics{
    pub window_pos: [u32;2],
    pub window_size: [u32;2],
    pub window_title: String,
    pub vsync: bool,
}

impl Default for Graphics{
    fn default() -> Self{
        Graphics{
            window_pos: [0,0],
            window_size: [800,600],
            window_title: "Tungsten".to_string(),
            vsync: false,
        }
    }
}

impl Settings{
    pub fn set_file<P: AsRef<Path>>(path: P){
        let file = io::load_wait_str(path.as_ref().clone());
        let file_res = file.get().unwrap().as_str();
        if let Ok(x) = file_res{
            (*SETTINGS.write().unwrap()) 
                = serde_json::from_str(&x).unwrap();
        }else if let Err(IOError::FileDoesNotExist) = file_res{
            let guard = &SETTINGS.read().unwrap();
            io::write_str(path,serde_json::to_string_pretty(guard.deref()).unwrap());
        }
    }
}
