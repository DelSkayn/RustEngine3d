use super::engine3d::Game;
use super::engine3d::obj;
use super::engine3d::render;
use super::engine3d::math::*;
use super::engine3d::render::camera;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct TestGame{
    mesh: render::RenderMesh,
    camera: camera::Camera,
}

impl Game for TestGame{
    fn new(renderengine: &render::RenderEngine) -> Self{
        let mut src = String::new();
        BufReader::new(
            File::open("res/teapot.obj").unwrap()
        ).read_to_string(&mut src).unwrap();
        let mesh = renderengine.create_mesh(
            &obj::ObjLoader::new(src).load().unwrap()
            ).unwrap();

        let mut cam = camera::Camera::with_perspective(90.0,800.0/600.0,0.1,10000.0);
//        let mut cam = Camera::new();
        cam.look_at(Vector3f::from_coords(0.0,0.0,10.0));


        TestGame{
            mesh: mesh,
            camera: cam,
        }
    }
    fn render<'a>(&'a mut self,mut que: render::RenderQueue<'a>) 
        -> render::RenderQueue<'a>{
        let ren_obj = render::RenderObject{
            mesh: &self.mesh,
            transform: Matrix4f::as_translation(Vector3f::from_coords(0.0,0.0,-1.0)),
        };

        que.queue.push(ren_obj);
        que.cam = self.camera.clone();
        que
    }

    fn update(&mut self){
    }
}
