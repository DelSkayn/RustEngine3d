
pub struct Vertex{
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    //text_pos: [f32; 2],
}

pub struct Mesh{
    pub indecies: Vec<u32>,
    pub vertecies: Vec<Vertex>,
    pub amount: usize,
}
