
use std::collections::HashMap;
use std::sync::RwLock;
use std::path::{Path,PathBuf};


mod asset_container;
pub use self::asset_container::*;

mod format;
pub use self::format::*;

use super::io::File;

lazy_static!(static ref ASSETS: RwLock<Assets> = RwLock::new(Assets::new()););

#[derive(Debug)]
pub enum Error{
    NoFileExtension,
    ExtensionUnknow,
}

pub enum Asset{
    Mesh(Container<Mesh>),
    Texture(Container<Texture>),
    Material(Container<Material>),
}

pub struct AssetData {
    name: String,
    path: PathBuf,
    data: Asset,
}

pub struct Assets{
    assets: HashMap<String,AssetData>,
    loaded: HashMap<PathBuf,String>,
}

impl Assets{
    fn new() -> Self{
        // Load defaults
        let assets = HashMap::new();
        let material: Material = Default::default();
        let mesh: Mesh = Default::default();
        let texture: Texture = Default::default();
        assets.push("default_texture".to_string(),AssetData{
            name: "default_texture".to_string(),
            path: Path::new("/").to_path_buf(),
            data: Asset::Texture(Container::new(texture))
        });
        assets.push("default_mesh".to_string(),AssetData{
            name: "default_mesh".to_string(),
            path: Path::new("/").to_path_buf(),
            data: Asset::Mesh(Container::new(mesh))
        });
        assets.push("default_material".to_string(),AssetData{
            name: "default_material".to_string(),
            path: Path::new("/").to_path_buf(),
            data: Asset::Material(Container::new(material))
        });
        Assets{
            assets: assets,
            loaded: HashMap::new(),
        }
    }

    fn load<S>(name: String,path: S) where S: AsRef<Path>{
        // check if asset is conflicting.
        let path_buf = path.as_ref().to_path_buf();
        {
            let borrow = ASSETS.read().expect("Asset lock poised");
            if borrow.assets.contains_key(&name){
                warn!("asset with name: \"{}\", already loaded",name);
                return;
            }
            if borrow.loaded.contains_key(&path_buf){
                warn!("asset at path: \"{}\", already loaded",path);
                return;
            }
        }
        // place asset data.
        let res = AssetData{
            name: name.clone(),
            path: path_buf,
            data: Asset,
        };
        {
            let borrow = ASSETS.write().expect("Asset lock poised");
            borrow.assets.insert(name.clone(),res);
            borrow.loaded.insert(path_buf.clone(),name);
        }
        // create load job.
        let file = File::open(path);
        let res = file.ready();
        match res { 
            Ok(_) => {
                match parse(name,path_buf,file){
                    Ok(_) => {},
                    Err(e) => warn!("Error loading asset \"{}\" because of file error \"{}\"",name,e),
                }
            },
            Err(e) => warn!("Error loading asset \"{}\" because of file error \"{}\"",name,e),
        }
    }
    
    fn get(name: String) -> Option<Asset>{
    }
}

// Asumes file is ready to use.
fn parse(name: String, path: PathBuf, file: File) -> Result<(),Error>{
    if let Some(x) = 
}

