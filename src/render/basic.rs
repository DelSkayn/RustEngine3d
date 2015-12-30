use super::super::glium::*;
use super::super::glium::backend::glutin_backend::GlutinFacade;
use super::*;
use super::mesh::Mesh;
use super::super::window::Window;
use std::rc::Rc;


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

pub struct BasicRenderer{
    context: GlutinFacade,
    shader: Program,
    meshes: Vec<Renderable>,
}

impl BasicRenderer{
    pub fn new(context: GlutinFacade,window: &Window) -> Self{
        trace!("RenderEngine Creation.");
        BasicRenderer{
            context: context,
            shader: Program::from_source(window.get_display(),&VS_SRC,&FS_SRC,None).unwrap(),
            meshes: Vec::new(),
        }
    }
}

impl RenderEngine for BasicRenderer{
    fn render(&mut self,renderque: RenderQueue,mut frame: Frame){
        trace!("Start rendering frame.");

        for robj in renderque.queue{
            let uniform = uniform!{
                _PerspectiveTransform: renderque.cam.get_perpective().as_array(),
                _CamTransform: renderque.cam.get_view().as_array(),
                _ModelTransform: robj.transform.as_array(),
            };
            let obj = &self.meshes[robj.mesh.index];
            frame.draw(&obj.vertex,&obj.index
                        ,&self.shader,&uniform
                        ,&Default::default()).unwrap();

        }
        frame.finish().unwrap();
        trace!("End rendering frame.");
    }

    fn create_mesh(&mut self,mesh: &Mesh) -> Result<RenderMesh,BufferError>{
        let vertex = try!(VertexBuffer::new(&self.context,&mesh.vertecies));
        let index = try!(IndexBuffer::new(&self.context ,index::PrimitiveType::TrianglesList
                                          ,&mesh.index));
        let index_mesh = self.meshes.len();
        self.meshes.push(Renderable{
            vertex: vertex,
            index: index,
        });
        Ok(RenderMesh{
            index: index_mesh,
        })
    }

    fn create_shader(&mut self,vs_src: String, fs_src: String, gs_src: Option<String>) -> Result<Program,ShaderCreationError>{
        Program::from_source(&self.context
                             ,&vs_src,&fs_src
                             ,gs_src.as_ref().map(|x| x as &str))
            .map_err(|x| ShaderCreationError::from(x))
    }
}
