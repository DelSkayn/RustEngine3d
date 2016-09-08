#![crate_type = "lib"]
#![allow(dead_code)]

extern crate crossbeam;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate tungsten_core;

use self::crossbeam::sync::MsQueue;

mod data;
mod asset_container;
mod format;
mod mesh;
mod metadata;
mod asset_map;

pub use self::asset_container::Container;
pub use self::format::{Texture,Material,Mesh};
use self::mesh::{MeshFileTypes,MeshLoader};
use self::asset_map::AssetMap;

use std::sync::RwLock;
use std::path::{Path};

use std::sync::atomic::{Ordering,AtomicUsize};

use tungsten_core::io::{File,CallbackResult};

lazy_static!{
    static ref ASSETS: RwLock<Assets> = RwLock::new(Assets::new());
    static ref ASSET_NEXT_ID: AtomicUsize = AtomicUsize::new(1);
    static ref PENDING: MsQueue<CallbackResult<()>> = MsQueue::new();
}

/// Error type which can be returned when loading assets
#[derive(Debug)]
pub enum Error{
    NoFileExtension,
    ExtensionUnknow,
}

/// A struct representing a id of a mesh.
#[derive(Clone,Copy,Eq,PartialEq,Hash)]
pub struct AssetId(usize);

/// Data about an asset.
pub struct AssetData<T> {
    name: String,
    id: AssetId,
    data: Container<T>,
}

impl<T> AssetData<T>{
    pub fn name(&self) -> &String{
        &self.name
    }

    pub fn id(&self) -> &AssetId{
        &self.id
    }

    pub fn data(&self) -> &Container<T>{
        &self.data
    }
}

impl<T> Clone for AssetData<T>{
    fn clone(&self) -> Self{
        AssetData{
            name: self.name.clone(),
            id: self.id.clone(),
            data: self.data.clone(),
        }
    }
}

/// Asset struct
/// The struct must be used with static function.
pub struct Assets{
    meshes: AssetMap<Mesh>,
    textures: AssetMap<Texture>,
    materials: AssetMap<Material>,
}

impl Assets{
    fn new() -> Self{
        // Load defaults
        let mut meshes = AssetMap::new();
        let mut textures = AssetMap::new();
        let mut materials = AssetMap::new();

        let material: Material = Default::default();
        let mesh: Mesh = Default::default();
        let texture: Texture = Default::default();

        textures.insert(AssetData{
            name: "default".to_string(),
            id: AssetId(0),
            data: Container::new(texture),
        });

        meshes.insert(AssetData{
            name: "default".to_string(),
            id: AssetId(0),
            data: Container::new(mesh),
        });

        materials.insert(AssetData{
            name: "default".to_string(),
            id: AssetId(0),
            data: Container::new(material),
        });

        Assets{
            textures: textures,
            meshes: meshes,
            materials: materials,
        }
    }

    /// Sets a mesh to be loaded with the given name from the given path.
    /// The file type will be determined by the extension of the file at the given path
    pub fn load_mesh<S,T>(name: T,path: S)
        where S: AsRef<Path>, T: Into<String>
        {
            Self::update_pending();

            let name = name.into();
            info!("Loading mesh \"{}\" at \"{}\".",name,path.as_ref().to_str().unwrap());
            // in case of a conflicting mesh name the function currently just returns
            // Weird actually. TODO: Need to come up with a better sollution
            if Self::conflicting_mesh(&name){
                warn!("asset with name: \"{}\", already loaded",name);
                return;
            }

            // create a container an place it in the
            let cont = Container::empty();
            Self::place_mesh(name,cont.clone());

            // create load job.
            let file = File::open(&path);
            match file{
                Ok(mut file) => {
                    // file was properly opened
                    if let Some(x) = path.as_ref().extension(){
                        if let Some(x) = MeshFileTypes::from_extension(x.to_str().unwrap()){
                            // could determin a proper file type
                            PENDING.push(file.read_to_end_callback(|data|{
                                MeshLoader::load(x,data,cont);
                            }));
                        }else{
                            warn!("Could not determin a implemented file type for mesh file \"{}\" with extension \"{}\".",path.as_ref().to_str().unwrap(),x.to_str().unwrap_or("Gawk! an error"));
                        }
                    }else{
                        warn!("Mesh file \"{}\", does not have an extension. Could not determin file type.",path.as_ref().to_str().unwrap());
                    }
                },
                Err(e) => warn!("Error loading asset because of file error \"{}\"",e),
            }
        }

    pub fn unload_mesh(name: &String){
        Self::update_pending();
        match ASSETS.write().expect("Asset lock poised").meshes.remove_name(name){
            Some(_) => {},
            None => warn!("Tried to remove asset which was not loaded."),
        }
    }

    pub fn get_mesh(name: &String) -> AssetData<Mesh>{
        Self::update_pending();
        let borrow = ASSETS.read().expect("Assets lock poised");
        match borrow.meshes.get_name(name){
            Some(x) => {
                x.clone()
            }
            None => {
                debug!("Could not find asset returning default.");
                borrow.meshes
                    .get(AssetId(0))
                    .unwrap()
                    .clone()
            }
        }
    }

    pub fn get_material(name: &String) -> AssetData<Material>{
        Self::update_pending();
        let borrow = ASSETS.read().expect("Assets lock poised");
        match borrow.materials.get_name(name){
            Some(x) => {
                x.clone()
            }
            None => {
                debug!("Could not find asset returning default.");
                borrow.materials
                    .get_name(&"default".to_string())
                    .unwrap()
                    .clone()
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
        if borrow.meshes.contains_key_name(name){
            return true;
        }
        return false
    }

    /// Places mesh asset data in its place.
    fn place_mesh(name: String,data: Container<Mesh>){
        let res = AssetData{
            name: name.clone(),
            id: AssetId(ASSET_NEXT_ID.fetch_add(1,Ordering::AcqRel)),
            data: data,
        };
        {
            let mut borrow = ASSETS.write().expect("Asset lock poised");
            borrow.meshes.insert(res);
        }
    }

    /// Update the pending callbacks.
    /// Should prob move to a scheduled task when implemented.
    fn update_pending(){
        while let Some(mut x) = PENDING.try_pop(){
            if let Some(r) = x.try(){
                match r{
                    Ok(_) => {},
                    Err(_) => warn!("Asset had error during loading"),
                }
            }else{
                PENDING.push(x);
                break;
            }
        }
    }


}

// Asumes file is ready to use.
