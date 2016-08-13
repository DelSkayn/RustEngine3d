

pub use super::*;

mod obj;
use self::obj::ObjLoader;

pub enum MeshFileTypes{
    Wavefront,
    Collada,
}

impl MeshFileTypes{
    pub fn from_extension(ext: &str) -> Option<Self>{
        match ext{
            "obj" => Some(MeshFileTypes::Wavefront),
            "dae" => Some(MeshFileTypes::Collada),
            _ => None,
        }
    }
}

pub struct MeshLoader;

impl MeshLoader{
    pub fn load(ty: MeshFileTypes,file: Vec<u8>,place: Container<Mesh>){
        match ty{
            MeshFileTypes::Wavefront => {
                ObjLoader::load(file,place);
            }
            MeshFileTypes::Collada => warn!("Collada mesh file not supported!"),
        }
    }
}
