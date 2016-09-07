extern crate cgmath;
use self::cgmath::*;

use super::super::glium::Surface;
use super::super::glium::Frame;
use super::super::glium::backend::Context;
use super::super::glium::program::Program;
use super::super::glium::draw_parameters::{DrawParameters,Depth,DepthTest};

use std::rc::Rc;

use tungsten_core::util::Cast;

use super::data;
use super::super::cache::Cache;

pub struct PipeLine{
    program: Program,
}

impl PipeLine{
    pub fn new(context: &Rc<Context>) -> Self{
        let program = Program::from_source(context
                                           ,data::VERTEX_SHADER
                                           ,data::FRAGMENT_SHADER
                                           ,None).unwrap();
        PipeLine{
            program: program,
        }
    }

    pub fn render(&self,cache: &Cache,frame: &mut Frame){
        for object in cache.que().iter(){
            let ref mesh = *object.mesh;
            let draw_para = DrawParameters{
                depth: Depth{
                    test: DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            };
            let p = PerspectiveFov{
                aspect: 800.0/600.0,
                fovy: Rad::from(Deg(90.0)),
                near: 0.1,
                far: 1000.0,
            }.into();
            let v: Matrix4<f32> = One::one();
            let m: Matrix4<f32> = Cast::<f32>::cast(object.transform.into());

            let mv = (v * m).clone();
            let mvp =  p.clone() * v * m;

            let uni = uniform!{MVPMat: mvp,MVMat: mv,PMat: p};
            frame.draw(&mesh.buffer,&mesh.index,&self.program,&uni,&draw_para).unwrap();
        }
    }
}
