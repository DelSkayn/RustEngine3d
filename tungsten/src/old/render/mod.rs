use super::glium::{
    VertexBuffer,
    IndexBuffer,
    Program,
    buffer,
    index,
    vertex,
    program,
};

//use super::glium::draw_parameters::PolygonMode;


use super::math::Matrix4f;

mod basic;
pub mod mesh;
pub mod camera;


use self::mesh::MeshVertex;
use self::mesh::Mesh;
pub use self::camera::Camera;
pub use self::basic::BasicRenderer; 


pub struct RenderMesh{
    vertex: VertexBuffer<MeshVertex>,
    index: IndexBuffer<u16>,
}

#[derive(Clone,Copy)]
pub struct RenderObject<'a>{
    pub mesh: &'a RenderMesh,
    pub transform: Matrix4f,
}

#[derive(Clone)]
pub struct RenderQueue<'a>{
    pub queue: Vec<RenderObject<'a>>,
    pub cam: Camera,
}

pub struct Shader{
    pub shader: Program,
    //source: String,
}

#[derive(Debug)]
pub enum BufferError{
    FormatNotSupported,
    OutOfMemory,
    BufferTypeNotSupported,
    IndexTypeNotSupported,
    PrimitiveTypeNotSupported,
}

impl From<vertex::BufferCreationError> for BufferError{
    fn from(err: vertex::BufferCreationError) -> Self{
        match err {
            vertex::BufferCreationError::FormatNotSupported => BufferError::FormatNotSupported,
            vertex::BufferCreationError::BufferCreationError(x) => match x{
                buffer::BufferCreationError::OutOfMemory => BufferError::OutOfMemory,
                buffer::BufferCreationError::BufferTypeNotSupported => BufferError::BufferTypeNotSupported,
            }
        }
    }
}

impl From<index::BufferCreationError> for BufferError{
    fn from(err: index::BufferCreationError) -> Self{
        match err {
            index::BufferCreationError::IndexTypeNotSupported=> BufferError::IndexTypeNotSupported,
            index::BufferCreationError::PrimitiveTypeNotSupported=> BufferError::PrimitiveTypeNotSupported,
            index::BufferCreationError::BufferCreationError(x) => match x{
                buffer::BufferCreationError::OutOfMemory => BufferError::OutOfMemory,
                buffer::BufferCreationError::BufferTypeNotSupported => BufferError::BufferTypeNotSupported,
            }
        }
    }
}

#[derive(Debug)]
pub enum ShaderCreationError{
    BinaryHeaderError,
    CompilationError(String),
    LinkingError(String),
    ShaderTypeNotSupported,
    CompilationNotSupported,
    TransformFeedbackNotSupported,
    PointSizeNotSupported,
}

impl From<program::ProgramCreationError> for ShaderCreationError{
    fn from(err: program::ProgramCreationError) -> Self{
        match err {
            program::ProgramCreationError::BinaryHeaderError => ShaderCreationError::BinaryHeaderError,
            program::ProgramCreationError::CompilationError(x) => ShaderCreationError::CompilationError(x),
            program::ProgramCreationError::LinkingError(x) => ShaderCreationError::LinkingError(x),
            program::ProgramCreationError::ShaderTypeNotSupported => ShaderCreationError::ShaderTypeNotSupported,
            program::ProgramCreationError::CompilationNotSupported => ShaderCreationError::CompilationNotSupported,
            program::ProgramCreationError::TransformFeedbackNotSupported => ShaderCreationError::TransformFeedbackNotSupported,
            program::ProgramCreationError::PointSizeNotSupported => ShaderCreationError::PointSizeNotSupported,
        }
    }
}


pub trait RenderEngine{
    fn render<'a>(&'a self, renderque: RenderQueue<'a>);

    fn create_mesh(&self,mesh: &Mesh)-> Result<RenderMesh,BufferError>;

    fn create_shader(&self,vs_src: String,
                     fs_src: String, 
                     gs_src: Option<String>) -> Result<Program,ShaderCreationError>;
}
