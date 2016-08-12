
use super::*;

struct Mesh{
    vertecies: Vec<f32>,
    normals: Vec<f32>,
    indecies: Vec<u32>,
}

impl Asset for Mesh{
    fn from_data(data: Vec<u8>, extension: String) -> Mesh{
        match extension.to_str() {
            "obj" => load_obj(data),
        }
    }
}
fn load_obj(data: Vec<u8>) -> Mesh{
