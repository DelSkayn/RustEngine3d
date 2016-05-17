use std::sync::RwLock;
use std::collections::HashMap;

use std::path::Path;
use std::path::PathBuf;

use std::any::Any;

lazy_static!{
    static ref SETTINGS: RwLock<Settings> 
        = RwLock::new(Default::default());
    static ref SETTINGS_FILE: RwLock<PathBuf> 
        = RwLock::new(Path::new("config/settings.json").to_path_buf());
}

#[derive(Serialize,Deserialize)]
pub struct Settings(HashMap<String,Box<Any + Sized>>);

unsafe impl Sync for Settings{}
unsafe impl Send for Settings{}

impl Settings{
    fn new() -> Self{
        Settings(HashMap::new());
    }

    pub fn get_self<T:Clone + Sync + Send + Sized,S: AsRef<str>>(&self,name: S) -> T{
        self.0.get(name.to_string())
            .expect("Setting \"{}\" does not exist",name)
            .downcast_ref()
            .expect("Setting \"{}\" does not have the correct type",name)
            .clone();
    }

    pub fn get<T:Clone + Sync + Send + Sized,S: AsRef<str>>(name: S) -> T{
        SETTINGS.read().get_self(name)
    }

    pub fn set<T:Clone + Sync + Send + Sized,S: AsRef<str>>(&self, name: S,value: T){
        self.0.insert(name,Box::new(value));
    }

    pub fn register(settings: Settings){
        *SETTINGS.write() = settings;
    }

    pub fn set_settings_file<P: AsRef<Path>>(path: P){
        *SETTINGS_FILE.write() = path.to_path_buf();
    }

    pub fn read_from_file(){
        unimplemented!();
    }
}

impl Default for Settings{
    fn default() -> Self{
        let res = Settings::new();
        res.set("window_size",[800u64,600u64]);
        res.set("window_pos" ,[0u64,0u64]);
        res.set("vsync",false);
        res
    }
}
