
use toml::Table;
use toml::Value;
use toml::Parser;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use std::sync::RwLock;

use std::result::Result as StdResult;

use std::path::Path;
use std::path::PathBuf; 
use io::Io;


mod register_type;
use self::register_type::RegistryType;

lazy_static!{
    static ref SETTINGS: RwLock<Registry> 
        = RwLock::new(Default::default());
    static ref SETTINGS_FILE: RwLock<PathBuf> 
        = RwLock::new(Path::new("./config/registry.toml").to_path_buf());
    static ref RUNNING: AtomicBool = AtomicBool::new(true);
}

#[derive(Debug)]
pub enum Error{
    EntryDoesntExist(String),
    InvalidType,
}

pub type Result<T> = StdResult<T,Error>;

pub struct Registry(Table);

impl Registry{
    fn new() -> Self{
        Registry(Table::new())
    }

    pub fn running() -> bool{
        RUNNING.load(Ordering::Acquire)
    }

    pub fn quit(){
        RUNNING.store(false,Ordering::Release);
    }

    pub fn get_self<T>(&self,name: &str) -> Result<T>
        where T: RegistryType,
    {
        Self::get_rec(&self.0,name)
    }

    fn get_rec<T>(table: &Table,name: &str) -> Result<T>
        where T: RegistryType,
    {
        if name.contains('.'){
            let (first,rest) = name.split_at(name.find('.').unwrap());
            let rest = &rest[1..rest.len()];
            match try!(table.get(first).ok_or(Error::EntryDoesntExist(name.to_string()))){
                &Value::Table(ref t) => {
                    Self::get_rec(t,rest)
                }
                _ => Err(Error::EntryDoesntExist(name.to_string())),
            }
        }else{
            T::from_value(
                try!(table.get(name)
                     .ok_or(Error::EntryDoesntExist(name.to_string())))
                ).ok_or(Error::InvalidType)
        }
    }

    pub fn get<T>(name: &str) -> Result<T>
        where T: RegistryType,
    {
        SETTINGS.read().expect("Registry lock poised!").get_self(name)
    }

    pub fn set_self<T>(&mut self, name: &str,value: T)
        where T: RegistryType,
    {
        let mut value = T::to_value(value);
        if name.contains('.'){
            let (first,rest) = name.split_at(name.find('.').unwrap());
            let rest = &rest[1..rest.len()];
            for s in rest.rsplit('.'){
                let mut new = Table::new();
                new.insert(s.to_string(),value);
                value = Value::Table(new);
            }
            self.0.insert(first.to_string(),value);
        }else{
            self.0.insert(name.to_string(),value);
        }
    }
    pub fn set<T>(name: &str,value: T)
        where T: RegistryType,
    {
        SETTINGS.write().unwrap().set_self(name,value)
    }

    pub fn set_full(registry: Registry){
        (*SETTINGS.write().expect("Registry lock poised!")) = registry;
    }

    pub fn set_file<P: AsRef<Path>>(path: P){
        (*SETTINGS_FILE.write().expect("Registry file path poised!")) = path.as_ref().to_path_buf();
    }

    pub fn read_from_file(){
        let path = SETTINGS_FILE.read().unwrap().clone();
        let res = Io::read(path.clone()).into_inner().map(|e| String::from_utf8(e).unwrap());
        match res{
            Ok(x) => {
                let mut parser = Parser::new(&x);
                let res = parser.parse();
                match res{
                    Some(x) => {
                        let mut s = SETTINGS.write().unwrap();
                        for (key,value) in x.into_iter(){
                            s.0.insert(key.clone(),value.clone());
                        }
                    },
                    None => {
                        warn!("Errors while parsing registry file: {:?}"
                               ,parser.errors);
                        return;
                    }
                }

            },
            Err(_) => {
                warn!("Could not find config file at path: {}",path.to_str().unwrap());
            }
        }
    }
}

impl Default for Registry{
    fn default() -> Self{
        let mut res = Registry::new();
        res.set_self("window.size",[800u64,600u64]);
        res.set_self("window.pos" ,[0u64,0u64]);
        res.set_self("window.title" ,"Tungsten".to_string());
        res.set_self("fullscreen" ,false);
        res.set_self("vsync",false);
        res
    }
}
