extern crate nalgebra;

use self::nalgebra::*;

use super::super::glium::Surface;
use super::super::glium::Frame;
use super::super::glium::backend::Context;
use super::super::glium::program::Program;
use super::super::glium::draw_parameters::{DrawParameters,Depth,DepthTest};

use std::rc::Rc;

use super::*;
use super::super::super::RenderObjects;
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

    pub fn render(&self,que: &RenderObjects,cache: &Cache,frame: &mut Frame){
        for (i,_) in que.iter().enumerate(){
            let mesh = cache.mesh(i);
            let draw_para = DrawParameters{
                depth: Depth{
                    test: DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            };
            let p = Perspective3::<f32>::new(800.0/600.0,2.0,0.1,1000.0).to_matrix();
            let v = Matrix4::<f32>::new_identity(4);
            let m = cache.transform(i).as_matrix();

            let mv = (v * m).clone();
            let mvp =  p.clone() * v * m;

            let a_mvp: [[f32;4];4] = mvp.as_ref().clone();
            let a_mv: [[f32;4];4] = mv.as_ref().clone();
            let a_p: [[f32;4];4] = p.as_ref().clone();

            let uni = uniform!{MVPMat: a_mvp,MVMat: a_mv,PMat: a_p};
            frame.draw(&mesh.buffer,&mesh.index,&self.program,&uni,&draw_para).unwrap();
        }
    }
}
