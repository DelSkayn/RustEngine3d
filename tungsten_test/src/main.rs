extern crate tungsten;

extern crate nalgebra;

use self::nalgebra::Vector3;

use self::tungsten::Game;
use self::tungsten::{Assets,Container,Mesh};
use self::tungsten::render::{Mutator,StaticRenderObject,Render,Transform};

struct TestGame{
    mesh: Container<Mesh>,
    render_handle: Option<Mutator<StaticRenderObject>>,
}

impl Game for TestGame{
    fn new() -> Self{
        Assets::load_mesh("sponza".to_string(),"res/models/sponza.obj");
        let mesh = Assets::get_mesh(&"sponza".to_string());
        mesh.wait();
        TestGame{
            mesh: mesh,
            render_handle: None,
        }
    }

    fn render(&mut self,render: &mut Render){
        let mut transform: Transform = Default::default();
        transform.translate(0.0,-3.0,-1.0);
        if self.render_handle.is_none(){
            let obj = StaticRenderObject{
                mesh: self.mesh.clone(),
                transform: transform, 
            };
            self.render_handle = Some(render.register(obj));
        }
        self.render_handle.as_mut()
            .unwrap()
            .borrow_mut()
            .transform
            .translate(-1.0,0.0,0.0);
    }
}


fn main(){
    tungsten::Engine::<TestGame>::Go();
}
