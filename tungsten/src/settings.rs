use super::serde_json;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use std::sync::RwLock;
use std::collections::HashMap;

use std::path::Path;
use std::path::PathBuf; 
use io::Io;

use std::str::FromStr;

lazy_static!{
    static ref SETTINGS: RwLock<Settings> 
        = RwLock::new(Default::default());
    static ref SETTINGS_FILE: RwLock<PathBuf> 
        = RwLock::new(Path::new("./config/settings.json").to_path_buf());
    static ref RUNNING: AtomicBool = AtomicBool::new(true);
}

pub trait SettingType: Sized{
    fn to_setting(&self) -> String;

    fn from_setting(s: &str) -> Option<Self>;
}

impl SettingType for u64 
{
    fn to_setting(&self) -> String{
        self.to_string()
    }

    fn from_setting(s: &str) -> Option<Self>{
        Self::from_str(s).ok()
    }
}

impl SettingType for i64 
{
    fn to_setting(&self) -> String{
        self.to_string()
    }

    fn from_setting(s: &str) -> Option<Self>{
        Self::from_str(s).ok()
    }
}

impl SettingType for u32 
{
    fn to_setting(&self) -> String{
        self.to_string()
    }

    fn from_setting(s: &str) -> Option<Self>{
        Self::from_str(s).ok()
    }
}

impl SettingType for i32 
{
    fn to_setting(&self) -> String{
        self.to_string()
    }

    fn from_setting(s: &str) -> Option<Self>{
        Self::from_str(s).ok()
    }
}

impl SettingType for bool
{
    fn to_setting(&self) -> String{
        self.to_string()
    }

    fn from_setting(s: &str) -> Option<Self>{
        Self::from_str(s).ok()
    }
}

impl SettingType for f32 
{
    fn to_setting(&self) -> String{
        self.to_string()
    }

    fn from_setting(s: &str) -> Option<Self>{
        Self::from_str(s).ok()
    }
}

impl SettingType for f64 
{
    fn to_setting(&self) -> String{
        self.to_string()
    }

    fn from_setting(s: &str) -> Option<Self>{
        Self::from_str(s).ok()
    }
}

impl SettingType for String 
{
    fn to_setting(&self) -> String{
        self.clone()
    }

    fn from_setting(s: &str) -> Option<Self>{
        Some(s.to_string())
    }
}

impl<T> SettingType for [T; 2]
    where T: SettingType,
{
    fn to_setting(&self) -> String{
        let mut res = self[0].to_setting();
        res.push_str(",");
        res.push_str(&self[1].to_setting());
        res
    }

    fn from_setting(s: &str) -> Option<[T; 2]>{
        let mut itt = s.split(',');
        if let Some(fir) = itt.next(){
            if let Some(sec) = itt.next(){
                if let Some(f) = T::from_setting(fir){
                    if let Some(s) = T::from_setting(sec){
                        return Some([f,s])
                    }
                }
            }
        }
        None
    }
}

#[derive(Serialize,Deserialize)]
pub struct Settings(HashMap<String,String>);

impl Settings{
    fn new() -> Self{
        Settings(HashMap::new())
    }

    pub fn running() -> bool{
        RUNNING.load(Ordering::Acquire)
    }

    pub fn quit(){
        RUNNING.store(false,Ordering::Release);
    }

    pub fn get_self<T,S>(&self,name: S) -> T
        where T: SettingType,
              S: AsRef<str>,
    {
        T::from_setting(self.0.get(&name.as_ref().to_string())
            .expect(&format!{"Setting \"{}\" does not exist",name.as_ref()}))
            .expect(&format!{"Setting \"{}\" does not have the correct type",name.as_ref()})
    }

    pub fn get<T,S>(name: S) -> T
        where T: SettingType,
              S: AsRef<str>,
    {
        SETTINGS.read().expect("Settings lock poised!").get_self(name)
    }

    pub fn set<T,S>(&mut self, name: S,value: T)
        where T: SettingType,
              S: AsRef<str>,
    {
        self.0.insert(name.as_ref().to_string(),value.to_setting());
    }

    pub fn register(settings: Settings){
        (*SETTINGS.write().expect("Settings lock poised!")) = settings;
    }

    pub fn set_settings_file<P: AsRef<Path>>(path: P){
        (*SETTINGS_FILE.write().expect("Settings file path poised!")) = path.as_ref().to_path_buf();
    }

    pub fn read_from_file(){
        let file = SETTINGS_FILE.read().unwrap().clone();
        let res = Io::read(file.clone())
            .into_inner();
        let data: Settings = match res{
            Ok(x) => {
                info!("Loading settings from file: \"{}\".",file.to_str().unwrap());
                serde_json::from_str(&String::from_utf8(x).unwrap()).unwrap()
            },
            Err(_) => {
                info!("could not find file \"{}\", creating it.",file.to_str().unwrap());
                let res = Default::default();
                Io::create(SETTINGS_FILE.read().unwrap().clone()
                          ,serde_json::to_vec_pretty(&res).unwrap()).into_inner().unwrap();
                res
            },
        };
        Self::register(data);
    }
}

impl Default for Settings{
    fn default() -> Self{
        let mut res = Settings::new();
        res.set("window_size",[800u64,600u64]);
        res.set("window_pos" ,[0u64,0u64]);
        res.set("window_title" ,"Tungsten".to_string());
        res.set("fullscreen" ,false);
        res.set("vsync",false);
        res
    }
}
