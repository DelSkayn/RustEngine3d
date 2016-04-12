//! 
//! This module handle game engine options. 
//! For now it is mostly a struct containing some data, 
//! but eventually it will also deal with loading a config file and managing runtime changes.
//!
//!

use std::sync::RwLock;

use std::convert::AsRef;

use std::default::Default;

use std::path::Path;

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


#[derive(Default,Serialize,Deserialize)]
pub struct SettingsInner{
    /// Options for the graphic part of the engine.
    pub graphics: Graphics,
}

impl Settings{
    pub fn set_file<P: AsRef<Path>>(path: P){
    }
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
