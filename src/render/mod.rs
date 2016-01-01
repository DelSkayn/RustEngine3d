use std::sync::Arc;
use super::glium::backend::glutin_backend::GlutinFacade;

use super::kernal::EventHandle;
use super::kernal::System;

use super::Event;

use std::fmt;

use super::glium::{
    VertexBuffer,
    IndexBuffer,
    Program,
    buffer,
    index,
    vertex,
    program,
    Frame,
};

//use super::glium::draw_parameters::PolygonMode;


use super::math::Matrix4f;

pub mod basic;
pub mod mesh;
pub mod camera;


use self::mesh::MeshVertex;
use self::mesh::Mesh;
pub use self::camera::Camera;
pub use self::basic::BasicRenderer; 

#[derive(Clone,Debug)]
pub enum RenderEvent{
    AddQueue(Arc<RenderQueue>),
    Frame,
    FrameDone,
}

pub struct Renderable{
    vertex: VertexBuffer<MeshVertex>,
    index: IndexBuffer<u16>,
}

pub struct RenderMesh{
    index: usize,
}

pub struct RenderObject{
    pub mesh: RenderMesh,
    pub transform: Matrix4f,
}


pub struct RenderQueue{
    pub queue: Vec<RenderObject>,
    pub cam: Camera,
}

impl fmt::Debug for RenderQueue{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"RenderQueue")
    }
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
            program::ProgramCreationError::CompilationError(x) => ShaderCreationError::CompilationError(x),
            program::ProgramCreationError::LinkingError(x) => ShaderCreationError::LinkingError(x),
            program::ProgramCreationError::ShaderTypeNotSupported => ShaderCreationError::ShaderTypeNotSupported,
            program::ProgramCreationError::CompilationNotSupported => ShaderCreationError::CompilationNotSupported,
            program::ProgramCreationError::TransformFeedbackNotSupported => ShaderCreationError::TransformFeedbackNotSupported,
            program::ProgramCreationError::PointSizeNotSupported => ShaderCreationError::PointSizeNotSupported,
        }
    }
}

pub struct RenderSystem<T: RenderEngine>{
    render_engine: T,
    event: EventHandle,
}

impl<T: RenderEngine> RenderSystem<T>{
    pub fn new(context: GlutinFacade,event: EventHandle) -> Self{
        RenderSystem{
            render_engine: T::new(context),
            event: event,
        }
    }
}

impl<T: RenderEngine> System for RenderSystem<T>{

    fn run(&mut self){
        for e in self.event.into_iter(){
            match e {
                Event::Render(x) => {
                    match x {
                        RenderEvent::Frame => self.render_engine.render(
                            RenderQueue{
                                queue: Vec::new(),
                                cam: Camera::new(),
                            }),
                        _ => {},
                    }
                },
                _ => {},
            }
        }
    }
}

pub trait RenderEngine{
    fn new(context: GlutinFacade) -> Self;

    fn render(&mut self, renderque: RenderQueue);

    fn create_mesh(&mut self,mesh: &Mesh)-> Result<RenderMesh,BufferError>;

    fn create_shader(&mut self,vs_src: String,
                     fs_src: String, 
                     gs_src: Option<String>) -> Result<Program,ShaderCreationError>;
}

