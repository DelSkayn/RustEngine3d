
use super::glium::*;

use std::rc::Rc;
use std::cell::RefCell;

use super::window::Window;
use super::mesh::Mesh;
use super::math::Matrix4f;


pub struct RenderMesh{
    vertex: VertexBuffer,
    normal: VertexBuffer,
    index: IndexBuffer,
}

#[derive(Copy,Debug)]
pub struct RenderObject<'a>{
    mesh: &'a RenderMesh,
    transform: Matrix4f,
}

pub struct RenderQueue<'a>{
    queue: Vec<RenderObject<'a>>,
    cam: Camera,
}

pub struct Shader{
    shader: Program,
    //source: String,
}

static vs_src:&'static str = & r#"
#version 320 

layout in vec3 position;
layout in vec3 normal;



"#

pub struct Shader{
    pub fn new() -> Self{
        Program{

        }
    }
}

pub struct RenderEngine{
    window: Rc<Window>,
}

impl RenderEngine{
    pub fn new(window: Rc<Window>) -> Self{
        RenderEngine{
            window: window,    
        }
    }

    pub fn render<'a>(&'a self,renderque: RenderQueue<'a>){
        let target = self.window.get_display().draw();
        let uniform = uniform!{
        }
        for obj in renderque.queue{

        }
    }

    pub fn create_mesh(&self,mesh: &Mesh) -> RenderMesh{
        let vertex = VertexBuffer::new(&self.window.get_display(),&mesh.vertex);
        let normal = VertexBuffer::new(&self.window.get_display(),&mesh.normal);
        let index = IndexBuffer::new(&self.window.get_display()
                                     ,index::PrimitiveType::TriangleList,&mesh.index);
        RenderMesh{
            vertex: vertex,
            normal: normal,
            index: index,
        }
    }
}
