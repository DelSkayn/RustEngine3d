extern crate cgmath;
use self::cgmath::*;

use tungsten_core::util::Cast;

#[derive(Clone,Copy)]
pub struct StaticRenderObject{
    pub pos: Point3<f64>,
    pub rot: Quaternion<f64>,
    pub scale: f64,
}

pub struct Camera{
    pos: Point3<f32>,
    look_at: Point3<f32>,
    up: Vector3<f32>,
    fov: f32,
    near: f32,
    far: f32,
}

impl Camera{
    pub fn as_transform_matrix(&self) -> Matrix4<f32>{
        Decomposed::<Vector3<f32>,Quaternion<f32>>::look_at(self.pos,self.look_at,self.up).into()
    }

    pub fn as_perspective_matrix(&self,aspect: f32) -> cgmath::Matrix4<f32>{
        PerspectiveFov{
            fovy: Deg(self.fov).into(),
            aspect: aspect,
            near: self.near,
            far: self.far,
        }.into()
    }

    pub fn as_matrix(&self, aspect: f32) -> cgmath::Matrix4<f32>{
        self.as_transform_matrix() * self.as_perspective_matrix(aspect)
    }
}
