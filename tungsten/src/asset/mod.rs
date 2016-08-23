extern crate crossbeam;

use self::crossbeam::sync::MsQueue;

use std::collections::HashMap;
use std::sync::RwLock;
use std::path::{Path};

mod asset_container;
pub use self::asset_container::*;

mod data;
mod mesh;
use self::mesh::{MeshFileTypes,MeshLoader};

mod format;
pub use self::format::*;

pub mod metadata;

pub use super::io::{File,CallbackResult};

lazy_static!(static ref ASSETS: RwLock<Assets> = RwLock::new(Assets::new()););


#[derive(Debug)]
pub enum Error{
    NoFileExtension,
    ExtensionUnknow,
}

pub enum AssetId{
    Mesh(String),
    Texture(String),
    Material(String),
} 
pub struct AssetData<T> {
    name: String,
    data: Container<T>,
}

pub struct Assets{
    meshes: HashMap<String,AssetData<Mesh>>,
    textures: HashMap<String,AssetData<Texture>>,
    materials: HashMap<String,AssetData<Material>>,
    pending: MsQueue<CallbackResult<()>>,
}

impl Assets{
    fn new() -> Self{
        // Load defaults
        let mut meshes = HashMap::new();
        let mut textures = HashMap::new();
        let mut materials = HashMap::new();
        let material: Material = Default::default();
        let mesh: Mesh = Default::default();
        let texture: Texture = Default::default();

        textures.insert("default".to_string(),AssetData{
            name: "default".to_string(),
            data: Container::new(texture),
        });

        meshes.insert("default".to_string(),AssetData{
            name: "default".to_string(),
            data: Container::new(mesh),
        });
        
        materials.insert("default".to_string(),AssetData{
            name: "default".to_string(),
            data: Container::new(material),
        });

        Assets{
            textures: textures,
            meshes: meshes,
            materials: materials,
            pending: MsQueue::new(),
        }
    }

    pub fn load_mesh<S>(name: String,path: S) where S: AsRef<Path>{
        info!("Loading mesh \"{}\" at \"{}\".",name,path.as_ref().to_str().unwrap());
        if Self::conflicting_mesh(&name){
            return;
        }
        let cont = Container::empty();
        Self::place_mesh(name,cont.clone());
        // create load job.
        let file = File::open(&path);
        match file{ 
            Ok(mut file) => {
                if let Some(x) = path.as_ref().extension(){
                    let borrow = ASSETS.read().expect("Asset lock poised");
                    if let Some(x) = MeshFileTypes::from_extension(x.to_str().unwrap()){
                        borrow.pending.push(file.read_to_end_callback(|data|{
                            MeshLoader::load(x,data,cont);
                        }));
                    }
                }else{
                    warn!("Mesh file \"{}\", does not have an extension. Could not determin file type.",path.as_ref().to_str().unwrap());
                }
            },
            Err(e) => warn!("Error loading asset because of file error \"{}\"",e),
        }
    }

    pub fn unload_mesh(name: &String){
        match ASSETS.write().expect("Asset lock poised").meshes.remove(name){
            Some(_) => {},
            None => warn!("Tried to remove asset which was not loaded."),
        }
    }

    pub fn get_mesh(name: &String) -> Container<Mesh>{
        let borrow = ASSETS.read().expect("Assets lock poised");
        match borrow.meshes.get(name){
            Some(x) => {
                x.data.clone()
            }
            None => {
                debug!("Could not find asset returning default.");
                borrow.meshes.get(&"default".to_string())
                    .as_ref().unwrap().data.clone()
            }
        }
    }

    pub fn get_material(name: &String) -> Container<Material>{
        let borrow = ASSETS.read().expect("Assets lock poised");
        match borrow.materials.get(name){
            Some(x) => {
                x.data.clone()
            }
            None => {
                debug!("Could not find asset returning default.");
                borrow.materials.get(&"default".to_string())
                    .as_ref().unwrap().data.clone()
            }
        }
    }

    pub fn load_material<S>(_name: String, _path: S) where S: AsRef<Path>{
        unimplemented!();
    }

    /// returns wether loading the mesh will result in conflicting
    /// ids or paths.
    fn conflicting_mesh(name: &String) -> bool{
        let borrow = ASSETS.read().expect("Asset lock poised");
        if borrow.meshes.contains_key(name){
            warn!("asset with name: \"{}\", already loaded",name);
            return true;
        }
        return false
    }

    /// Places mesh asset data in its place.
    fn place_mesh(name: String,data: Container<Mesh>){
        let res = AssetData{
            name: name.clone(),
            data: data,
        };
        {
            let mut borrow = ASSETS.write().expect("Asset lock poised");
            borrow.meshes.insert(name,res);
        }
    }

}

// Asumes file is ready to use.
