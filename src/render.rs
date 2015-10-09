use super::glium::*;
//use super::glium::draw_parameters::PolygonMode;

use std::rc::Rc;

use super::window::Window;
use super::mesh::Mesh;
use super::mesh::MeshVertex;
use super::math::Matrix4f;
use super::camera::Camera;

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
    shader: Program,
    //source: String,
}

static VS_SRC:&'static str = & r#"
#version 330 

in vec3 position;
in vec3 normal;

uniform mat4 _ModelTransform;
uniform mat4 _CamTransform;
uniform mat4 _PerspectiveTransform;

out VS_OUT{
    vec3 color;
} vs_out;

void main(){
    vs_out.color = normal;

    vec4 p = vec4(position,1.0);
    mat4 trans =    _PerspectiveTransform * _CamTransform * _ModelTransform;

    gl_Position = trans * p;
}

"#;

static FS_SRC:&'static str = & r#"
#version 330 

in VS_OUT{
    vec3 color;
} fs_in;

out vec4 color;

void main(){
    color = vec4(fs_in.color+0.1,1.0);
}
"#;

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

pub struct RenderEngine{
    window: Rc<Window>,
    shader: Program,
}

impl RenderEngine{
    pub fn new(window: Rc<Window>) -> Self{
        RenderEngine{
            shader: Program::from_source(window.get_display(),&VS_SRC,&FS_SRC,None).unwrap(),
            window: window,    
        }
    }

    pub fn render<'a>(&'a self,renderque: RenderQueue<'a>){
        let mut target = self.window.get_display().draw();

        for obj in renderque.queue{
        let uniform = uniform!{
            _PerspectiveTransform: renderque.cam.get_perpective().as_array(),
            _CamTransform: renderque.cam.get_view().as_array(),
            _ModelTransform: obj.transform.as_array(),
        };
            target.draw(&obj.mesh.vertex,&obj.mesh.index
                        ,&self.shader,&uniform
                        ,&Default::default()).unwrap();
            
        }
        target.finish().unwrap();
    }

    pub fn create_mesh(&self,mesh: &Mesh) -> Result<RenderMesh,BufferError>{
        let vertex = try!(VertexBuffer::new(self.window.get_display(),&mesh.vertecies));
        let index = try!(IndexBuffer::new(self.window.get_display()
                                     ,index::PrimitiveType::TrianglesList,&mesh.index));
        Ok(RenderMesh{
            vertex: vertex,
            index: index,
        })
    }

    pub fn create_shader(&self,vs_src: String, fs_src: String, gs_src: Option<String>) -> Result<Program,ShaderCreationError>{
        Program::from_source(self.window.get_display()
                             ,&vs_src,&fs_src
                             ,gs_src.as_ref().map(|x| x as &str))
                                .map_err(|x| ShaderCreationError::from(x))
    }
}
