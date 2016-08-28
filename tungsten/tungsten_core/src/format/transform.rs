
use super::nalgebra::{UnitQuaternion,ToHomogeneous,Rotation,Isometry3};
use super::*;

#[derive(Clone,Copy,Debug)]
pub struct Transform{
    pub position: Point3,
    pub rotation: Quaternion,
    pub scale: Float,
}

impl Transform{
    pub fn new(pos: Point3,orientation: Vector3,scale: Float) -> Self{ 
        Transform{
            position: pos,
            rotation: UnitQuaternion::from_scaled_axis(orientation),
            scale: scale,
        }
    }

    pub fn as_matrix(&self) -> Matrix4{
        Isometry3::new(self.position.to_vector()
                         ,self.rotation.rotation())
            .to_homogeneous()
    }

    pub fn translate(&mut self,trans: Vector3) -> &mut Self{
        self.position += trans;
        self
    }

    pub fn rotate(&mut self,rot: Quaternion) -> &mut Self{
        self.rotation *= rot;
        self
    }

    pub fn scale(&mut self,scale: Float) -> &mut Self{
        self.scale *= scale;
        self
    }

    pub fn set_position(&mut self,pos: Point3) -> &mut Self{
        self.position = pos;
        self
    }

    pub fn set_rotation(&mut self,rot: Quaternion) -> &mut Self{
        self.rotation = rot;
        self
    }

    pub fn set_scale(&mut self,scale: Float) -> &mut Self{
        self.scale = scale;
        self
    }
}

impl Default for Transform{
    fn default() -> Self{
        Transform{
            position: Point3::new(0.0,0.0,0.0),
            rotation: UnitQuaternion::from_euler_angles(0.0,0.0,0.0),
            scale: 1.0,
        }
    }
}

