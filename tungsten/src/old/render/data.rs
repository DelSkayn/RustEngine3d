
pub struct Texture{
    size: [u32; 2],
    data: Vec<u8>,
}

pub struct Material{
    metalness: f32,
    roughness: f32,
    //TODO change to [u8; 3]
    //and confert when nessecary.
    spec_color: [f32; 3],
    texture: Option<Texture>,
}

pub struct Mesh{
    vertex: Vec<[f32; 3]>,
    normal: Vec<[f32; 3]>,
    index: Vec<u32>,
    material: Material,
}
