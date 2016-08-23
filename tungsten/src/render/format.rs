extern crate nalgebra;
use self::nalgebra::{Perspective3,UnitQuaternion,Vector3,Matrix4};

use asset::{Container, Mesh};

pub struct StaticRenderObject{
    pub mesh: Container<Mesh>,
    pub transform: Matrix4<f32>,
}

pub struct Camera{
    pub perspective: Perspective3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub translation: Vector3<f32>,
}

pub struct RenderQue{
    pub layers: Vec<Layer>,
}

pub struct Layer{
    pub camera: Camera,
    pub static_mesh: Vec<()>,
}

impl Default for Camera{
    fn default() -> Self{
        Camera{
            perspective: Perspective3::new(800.0/600.0,2.0,0.1,1000.0),
            rotation: UnitQuaternion::new(Vector3::new(0.0,0.0,1.0)),
            translation: Vector3::new(0.0,0.0,0.0),
        }
    }
}
