extern crate num_cpus;

pub enum SystemType{
    Bit32,
    Bit64,
}

pub enum OperatingSystem{
    Windows,
    Linux,
    MacOs,
    Other,
}

/// Struct containing data about the platform the engine is running on
pub struct Platform{
    /// The current operating system
    pub os: OperatingSystem,
    /// Amount of cpu cores, Physical and logical(?).
    pub cores: usize,
}

impl Platform{
    /// Creates a new platform.
    pub fn new() -> Self{
        let os =
        if cfg!(target_os = "linux"){
            OperatingSystem::Linux
        }else if cfg!(target_os = "windows"){
            OperatingSystem::Windows
        }else if cfg!(target_os = "macos"){
            OperatingSystem::MacOs
        }else{
            OperatingSystem::Other
        };
        
        Platform{
            os: os,
            cores: num_cpus::get(),
        }
    }
}
