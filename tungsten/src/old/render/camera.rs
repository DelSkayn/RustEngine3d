
use super::super::math::*;
use std::f32::consts;

#[derive(Clone,Copy,Debug)]
pub struct Camera{
    perspective: Matrix4f,
    rotation: Quatf,
    position: Vector3f,
}

impl Camera{
    pub fn new() -> Self{
        Camera{
            perspective: Matrix4f::identity(),
            rotation: Quatf::new(),
            position: Vector3f::new(),
        }
    }

    pub fn set_position(&mut self,pos: Vector3f){
        self.position = pos;
    }

    pub fn with_perspective(fov: f32,aspect_ratio: f32,z_near: f32,z_far: f32) -> Self{
        Camera{
            perspective: Matrix4f::as_perspective(fov,aspect_ratio,z_near,z_far),
            rotation: Quatf::new(),
            position: Vector3f::new(),
        }
    }

    pub fn get_perpective(&self) -> Matrix4f{
        self.perspective
    }

    pub fn get_view(&self) -> Matrix4f{
        let mat = Matrix4f::as_translation(self.position);
        (mat * self.rotation.to_matrix()).invert()
    }

    pub fn look_at(&mut self, at: Vector3f){
        let forward = (at-self.position).normalize();
        let dot = Vector3f::from_coords(0.0,0.0,1.0).dot(&forward);

        if(dot + 1.0).abs() < 0.000001{
            self.rotation = Quatf::from_angle(Vector3f::from_coords(0.0,-1.0,0.0),consts::PI);
        }else if(dot - 1.0).abs() < 0.000001{
            self.rotation = Quatf::new();
        }else{
            let angle = dot.acos();
            let axis = Vector3f::from_coords(0.0,0.0,1.0).cross(&forward).normalize();
            self.rotation = Quatf::from_angle(axis,angle);
        }
    }
}

