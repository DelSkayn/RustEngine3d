extern crate nalgebra;

use self::nalgebra::{Matrix4};

use super::glium::backend::Context;
use super::glium::{VertexBuffer,IndexBuffer};
use super::glium::buffer::BufferCreationError;
use super::glium::vertex::BufferCreationError as VertexBufCreationError;
use super::glium::index::{PrimitiveType, BufferCreationError as IndexBufCreationError};

use std::rc::Rc;

use asset::Mesh;


#[derive(Debug)]
pub enum OglError{
    Buffer(BufferError),
}

#[derive(Debug)]
pub enum BufferError{
    OutOfMemory,
    BufferTypeNotSupported,
    FormatNotSupported,
    IndexTypeNotSupported,
    PrimitiveTypeNotSupported,
}

#[derive(Copy,Clone)]
pub struct Vertex{
    position: [f32; 3],
    normals: [f32; 3],
}

pub struct StaticMeshNoTextureData{
    pub transform: Matrix4<f32>,
    pub mesh: StaticMeshNoTexture,
}

pub type StaticMeshNoTextureQue = Vec<StaticMeshNoTextureData>;

implement_vertex!(Vertex,position,normals);

pub struct StaticMeshNoTexture{
    pub buffer: VertexBuffer<Vertex>,
    pub index: IndexBuffer<u32>,
}

impl StaticMeshNoTexture{
    pub fn from_mesh(context: Rc<Context>,mesh: &Mesh) -> Result<Self,OglError>{
        let mut normals = mesh.normals.iter();
        let vertecies: Vec<_> = mesh.vertecies.iter()
            .map(|e|{
                Vertex{
                    position: e.clone(),
                    normals: normals.next().unwrap().clone()
                }
            }).collect();


        let vert_buf = try!(VertexBuffer::new(&context,&vertecies));
        let index_buf = try!(IndexBuffer::new(&context,PrimitiveType::TrianglesList,&mesh.indecies));

        Ok(StaticMeshNoTexture{
            buffer: vert_buf,
            index: index_buf,
        })
    }
}

impl From<BufferCreationError> for OglError{
    fn from(e: BufferCreationError) -> Self{
        OglError::Buffer(BufferError::from(e))
    }
}

impl From<VertexBufCreationError> for OglError{
    fn from(e: VertexBufCreationError) -> Self{
        OglError::Buffer(BufferError::from(e))
    }
}

impl From<IndexBufCreationError> for OglError{
    fn from(e: IndexBufCreationError) -> Self{
        OglError::Buffer(BufferError::from(e))
    }
}

impl From<VertexBufCreationError> for BufferError{
    fn from(e: VertexBufCreationError) -> Self{
        match e{
            VertexBufCreationError::FormatNotSupported => BufferError::FormatNotSupported,
            VertexBufCreationError::BufferCreationError(e) => {
                BufferError::from(e)
            },
        }
    }
}

impl From<BufferCreationError> for BufferError{
    fn from(e: BufferCreationError) -> Self{
        match e {
            BufferCreationError::OutOfMemory => BufferError::OutOfMemory,
            BufferCreationError::BufferTypeNotSupported => BufferError::BufferTypeNotSupported,
        }
    }
}

impl From<IndexBufCreationError> for BufferError{
    fn from(e: IndexBufCreationError) -> Self{
        match e {
            IndexBufCreationError::IndexTypeNotSupported => BufferError::IndexTypeNotSupported,
            IndexBufCreationError::PrimitiveTypeNotSupported => BufferError::PrimitiveTypeNotSupported,
            IndexBufCreationError::BufferCreationError(e) => {
                BufferError::from(e)
            },
        }
    }
}
