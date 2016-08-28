extern crate nalgebra;

use tungsten_core::format::Transform;

use self::nalgebra::{Perspective3,Vector3,Matrix4,ToHomogeneous,Isometry3,Point3,Cast};

#[derive(Clone,Copy)]
pub struct StaticRenderObject{
    pub transform: Transform,
}

pub struct Camera{
    pos: Point3<f64>,//f64 to preserve presision
    look_at: Point3<f64>,
    up: Vector3<f32>,
    fov: f32,
    near: f32,
    far: f32,
}

impl Camera{
    pub fn as_transform_matrix(&self) -> Matrix4<f32>{
        let pos = Cast::<Point3<f64>>::from(self.pos);
        let look_at = Cast::<Point3<f64>>::from(self.look_at);
        Isometry3::new_observer_frame(&pos,&look_at,&self.up).to_homogeneous()
    }

    pub fn as_perspective_matrix(&self,aspect: f32) -> Matrix4<f32>{
        Perspective3::new(aspect,self.fov,self.near,self.far).to_matrix()
    }

    pub fn as_matrix(&self, aspect: f32) -> Matrix4<f32>{
        self.as_transform_matrix() * self.as_perspective_matrix(aspect)
    }
}
