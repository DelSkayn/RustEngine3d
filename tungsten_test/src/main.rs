extern crate tungsten;

extern crate nalgebra;

use self::tungsten::Game;
use self::tungsten::render::{Render,RegisteredObject,StaticRenderObject};
use self::tungsten::asset::Assets;
use self::tungsten::core::console::{Console,Terminal};
use self::tungsten::core::format::{Vector3,Transform,Quaternion};

struct TestGame{
    render_handle: RegisteredObject,
}

impl Game for TestGame{
    fn new<T: Terminal>(render: &mut Render, _: &mut Console<T>) -> Self{
        Assets::load_mesh("sponza".to_string(),"res/models/teapot.obj");
        let mesh = Assets::get_mesh(&"sponza".to_string());
        mesh.data().wait();
        let mut transform: Transform = Default::default();
        transform.translate(Vector3::new(0.0,0.0,-2.0));
        let obj = StaticRenderObject{
            mesh: mesh.data().clone(),
            transform: transform, 
        };
        let render_handle = render.register(obj);
        TestGame{
            render_handle: render_handle,
        }
    }

    fn update(&mut self){
        self.render_handle
            .borrow_mut()
            .transform
            .rotate(Quaternion::from_axisangle(nalgebra::Unit::new(&Vector3::new(0.0,0.0,1.0)),0.02));
    }
}


fn main(){
    tungsten::Engine::<TestGame>::Go();
}
