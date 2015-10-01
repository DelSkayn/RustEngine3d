
use super::math::*;

#[derive(Copy,Debug)]
pub struct Camera{
    perspective: Matrix4f,
    rotation: Quatf,
    position: Vector3f,
}

impl Camera{
    pub fn new() -> Self{
        Camera{
            perspective: Matrix4::new(),
            rotation: Quatf::new(),
            position: Vector2f::new(),
        }
    }

    pub fn with_perspective(fov: f32,aspect_ratio: f32,z_far: f32,z_near: f32) -> Self{
        Camera{
            perspective: Matrix4::as_perspective(fov,aspect_ratio,z_far,z_near),
            rotation: Quatf::new(),
            position: Vector2f::new(),
        }
    }
}

