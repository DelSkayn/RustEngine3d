use std::sync::Arc;

pub struct StaticMesh{
    pub data: Arc<()>,
}

pub struct RenderQue{
    pub static_mesh: Vec<StaticMesh>,
}
