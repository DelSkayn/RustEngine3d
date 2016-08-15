use std::sync::Arc;

pub struct Mesh;

pub struct StaticRenderObject{
    pub mesh: Mesh,
}

pub struct RenderQue{
    pub static_mesh: Vec<StaticRenderObject>,
}
