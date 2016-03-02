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

pub struct Platform{
    pub os: OperatingSystem,
    pub cores: usize,
}

impl Platform{

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
