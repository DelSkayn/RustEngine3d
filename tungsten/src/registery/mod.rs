//! The registry
//!
//! The registry is the place where settings are stored in the engine.
//! It holds things like window size, Rendering device, And other settings.
//! Can also be used by the user for game settings like Keybindings and gameplay options.
//!
//! The registry can be loaded from a file in the TOML format.
//! it currently only loads from the file and is not able to write the settings back.
//! This might changes some day but for the close future i do not see it happening quickley.
//!
//! ### TODO:
//!
//!     * Move running out of registry.
//!     * Remove the `SETTINGS_FILE` var since the file can now only be used for loading.
//!     * Add a way to notify systems of setting changes.
//!     * Add a way to split settings into multiple files in different locations.
//!

use toml::Table;
use toml::Value;
use toml::Parser;


use std::sync::RwLock;

use std::result::Result as StdResult;

use std::path::{Path,PathBuf};

use io::File;


mod register_type;
use self::register_type::RegisteryType;

lazy_static!{
    static ref SETTINGS: RwLock<Registery> 
        = RwLock::new(Default::default());
    static ref SETTINGS_FILE: RwLock<PathBuf> 
        = RwLock::new(Path::new("./config/registery.toml").to_path_buf());
}

#[derive(Debug)]
pub enum Error {
    EntryDoesntExist(String),
    InvalidType,
}

#[derive(Debug)]
pub enum RegResult<T>{
    Ok(T),
    Err(Error),
}

impl<T> RegResult<T>{
    pub fn new(val: StdResult<T,Error>) -> Self{
        match val{
            Ok(x) => RegResult::Ok(x),
            Err(x) => RegResult::Err(x),
        }
    }

    pub fn unwrap(self) -> T{
        if let RegResult::Ok(x) = self{
            x
        }else{
            panic!();
        }
    }
    
    pub fn is_ok(&self) -> bool{
        if let &RegResult::Ok(_) = self{
            true
        }else{
            false
        }
    }

    pub fn is_err(&self) -> bool{
        if let &RegResult::Err(_) = self{
            true
        }else{
            false
        }
    }

    pub fn or(self,default: T) -> T{
        if let RegResult::Ok(x) = self{
            x
        }else{
            default
        }
    }
}

impl <T: Default> RegResult<T>{
    pub fn or_default(self) -> T{
        if let RegResult::Ok(x) = self{
            x
        }else{
            Default::default()
        }
    }
}

pub type Result<T> = RegResult<T>;

pub struct Registery(Table);

impl Registery {
    fn new() -> Self {
        Registery(Table::new())
    }

    pub fn get_self<T>(&self, name: &str) -> Result<T>
        where T: RegisteryType
    {
        Self::get_rec(&self.0, name)
    }

    fn get_rec<T>(table: &Table, name: &str) -> Result<T>
        where T: RegisteryType
    {
        if name.contains('.') {
            let (first, rest) = name.split_at(name.find('.').unwrap());
            let rest = &rest[1..rest.len()];
            let res = table.get(first).ok_or(Error::EntryDoesntExist(name.to_string()));
            if let Err(x) = res{
                return RegResult::Err(x);
            }
            match *res.unwrap() {
                Value::Table(ref t) => Self::get_rec(t, rest),
                _ => RegResult::new(Err(Error::EntryDoesntExist(name.to_string()))),
            }
        } else {
            let res = table.get(name).ok_or(Error::EntryDoesntExist(name.to_string()));
            if let Err(x) = res{
                return RegResult::Err(x);
            }
            RegResult::new(T::from_value(res.unwrap()).ok_or(Error::InvalidType))
        }
    }

    pub fn get<T>(name: &str) -> Result<T>
        where T: RegisteryType
    {
        SETTINGS.read().expect("Registery lock poised!").get_self(name)
    }

    pub fn set_self<T>(&mut self, name: &str, value: T)
        where T: RegisteryType
    {
        let mut value = T::to_value(value);
        if name.contains('.') {
            let (first, rest) = name.split_at(name.find('.').unwrap());
            let rest = &rest[1..rest.len()];
            for s in rest.rsplit('.') {
                let mut new = Table::new();
                new.insert(s.to_string(), value);
                value = Value::Table(new);
            }
            self.0.insert(first.to_string(), value);
        } else {
            self.0.insert(name.to_string(), value);
        }
    }
    pub fn set<T>(name: &str, value: T)
        where T: RegisteryType
    {
        SETTINGS.write().unwrap().set_self(name, value)
    }

    pub fn set_full(registry: Registery) {
        (*SETTINGS.write().expect("Registery lock poised!")) = registry;
    }

    pub fn set_file<P: AsRef<Path>>(path: P) {
        (*SETTINGS_FILE.write().expect("Registery file path poised!")) = path.as_ref().to_path_buf();
    }

    pub fn read_from_file() {
        let path = SETTINGS_FILE.read().unwrap().clone();
        info!("Reading config file at: {}",path.to_str().unwrap());
        let file = File::open(&path);
        if let Ok(mut x) = file{
            let res = x.read_to_end().wait()
                .map(|e| String::from_utf8(e).unwrap());
            match res {
                Ok(x) => {
                    let mut parser = Parser::new(&x);
                    let res = parser.parse();
                    match res {
                        Some(x) => {
                            let mut s = SETTINGS.write().unwrap();
                            for (key, value) in x.into_iter() {
                                s.0.insert(key.clone(), value.clone());
                            }
                        }
                        None => {
                            warn!("Errors while parsing registry file: {:?}", parser.errors);
                            return;
                        }
                    }

                }
                Err(e) => {
                    warn!("Could not read config file. reason : {:?}",e);
                }
            }
        }else{
            warn!("Log file not found, using default.");
        }
    }
}

impl Default for Registery {
    fn default() -> Self {
        let mut res = Registery::new();
        res.set_self("window.size", [800u64, 600u64]);
        res.set_self("window.pos", [0u64, 0u64]);
        res.set_self("window.title", "Tungsten".to_string());
        res.set_self("window.fullscreen", false);
        res.set_self("window.vsync", true);
        res.set_self("general.quit_on_esc", true);
        res
    }
}
