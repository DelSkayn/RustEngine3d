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
    pub static_mesh: Vec<StaticRenderObject>,
    pub camera: Camera,
}
