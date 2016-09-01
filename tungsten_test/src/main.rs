extern crate tungsten;

extern crate nalgebra;

extern crate rand;

use self::tungsten::task::{self,Config,config};
use self::tungsten::Game;
use self::tungsten::render::{Render,RegisteredObject,StaticRenderObject};
use self::tungsten::asset::Assets;
use self::tungsten::core::console::{Console,Terminal};
use self::tungsten::core::format::{Vector3,Transform,Quaternion};

struct TestGame{
    render_handle: Vec<RegisteredObject>,
}

impl Game for TestGame{
    fn new<T: Terminal>(render: &mut Render, _: &mut Console<T>) -> Self{
        Assets::load_mesh("sponza".to_string(),"res/models/teapot.obj");
        let mesh = Assets::get_mesh(&"sponza".to_string());
        mesh.data().wait();
        let mut data = Vec::new();
        for _ in 0..500{
            let mut transform: Transform = Default::default();
            let x = (rand::random::<i64>() % 100) as f64;
            let y = (rand::random::<i64>() % 100) as f64;
            let z = ((rand::random::<i64>() % 100) - 150) as f64;
            transform.translate(Vector3::new(x/10.0,y/10.0,z/10.0));
            let obj = StaticRenderObject{
                transform: transform, 
            };
            data.push(render.register(obj,mesh.clone()));
        }
        TestGame{
            render_handle: data
        }
    }

    fn update(&mut self){
        up(&mut self.render_handle);
    }
}

fn up(data: &mut [RegisteredObject]){
    if data.len() == 1{
        data[0].borrow_mut()
            .transform
            .rotate(Quaternion::from_axisangle(nalgebra::Unit::new(&Vector3::new(0.0,0.0,1.0)),0.02));
    }else{
        let len = data.len() / 2;
        let (first,rest) = data.split_at_mut(len);
        task::join(|| up(first),|| up(rest));
    }
}


fn main(){
    tungsten::Engine::<TestGame>::Go();
}
