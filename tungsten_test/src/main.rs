extern crate tungsten;

extern crate nalgebra;

use self::tungsten::Game;
use self::tungsten::{Render,Assets,Console,Terminal};
use self::tungsten::{RegisteredObject,StaticRenderObject,Transform};

struct TestGame{
    render_handle: RegisteredObject,
}

impl Game for TestGame{
    fn new<T: Terminal>(render: &mut Render, _: &mut Console<T>) -> Self{
        Assets::load_mesh("sponza".to_string(),"res/models/sponza.obj");
        let mesh = Assets::get_mesh(&"sponza".to_string());
        mesh.wait();
        let mut transform: Transform = Default::default();
        transform.translate(0.0,-3.0,-1.0);
        let obj = StaticRenderObject{
            mesh: mesh,
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
            .translate(1.0,-1.0,0.0);
    }
}


fn main(){
    tungsten::Engine::<TestGame>::Go();
}
