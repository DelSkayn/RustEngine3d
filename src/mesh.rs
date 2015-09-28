
#[derive(Copy,Clone)]
pub struct MeshVertex{
    pub position: [f32; 3],
    pub normal: [f32; 3],
    //pub color: [f32; 3],
    //pub texture_pos: [f32; 2],
}

implement_vertex!(MeshVertex,position,normal);

pub struct Mesh<I>{
    pub vertecies: Vec<MeshVertex>,
    pub index: Vec<I>,
}
