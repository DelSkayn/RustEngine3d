//! 
//! This module handle game engine options. 
//! For now it is mostly a struct containing some data, 
//! but eventually it will also deal with loading a config file and managing runtime changes.
//!

///
/// A struct containing all the settings in the engine divided in catagories as 
/// seen in option menus.
///
pub struct Settings{
    /// Options for the graphic part of the engine.
    pub graphics: Graphics,
}

impl Settings{
    /// Creates a new setting struct.
    pub fn new() -> Self{
        Settings{
            graphics: Default::default(),
        }
    }
}

/// struct containing options for the graphic part of the engine.
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
