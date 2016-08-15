
use super::Container;

use super::data::*;

pub struct Mesh{
    pub vertecies: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub texture_coords: Option<Vec<[f32; 2]>>,
    pub indecies: Vec<usize>,
    pub material: Container<Material>,
}

pub struct Material{
    pub roughness: f32,
    pub metalness: f32,
    pub spec_color: [f32; 3],
    pub diffuse: [f32; 3],
    pub diffuse_map: Option<Container<Texture>>,
    pub roughness_map: Option<Container<Texture>>,
    pub metalness_map: Option<Container<Texture>>,
    pub bump_map: Option<Container<Texture>>,
}

enum TextureFormat{
    RGBA,
    RGB,
}

pub struct Texture{
    data: Vec<u8>,
    size: [u32; 2],
    format: TextureFormat,
}

impl Default for Mesh{
    fn default() -> Self{
        let vertecies = SPHERE_VERTECIES.chunks(3).map(|e| [e[0],e[1],e[2]]).collect();
        let normals = SPHERE_NORMALS.chunks(3).map(|e| [e[0],e[1],e[2]]).collect();
        Mesh{
            vertecies: vertecies,
            normals: normals,
            indecies: SPHERE_INDECIES.to_vec(),
            texture_coords: None,
            material: Container::new(Default::default()),
        }
    }
}

impl Default for Material{
    fn default() -> Self{
        Material{
            roughness: 0.5,
            metalness: 0.5,
            spec_color: [0.5,0.5,0.0],
            diffuse: [0.0,0.0,1.0],
            diffuse_map: None,
            roughness_map: None,
            metalness_map: None,
            bump_map: None,
        }
    }
}

impl Default for Texture{
    fn default() -> Self{
        Texture{
            data: vec![0,255,0],
            size: [1,1],
            format: TextureFormat::RGB,
        }
    }
}
