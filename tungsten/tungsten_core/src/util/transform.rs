extern crate nalgebra;
use self::nalgebra::*;

/// Struct detailing the trans form struct.
/// Use cgmath `Decomposed` instead.
#[derive(Clone,Debug)]
pub struct Transform{
    pub rotation: UnitQuaternion<f64>,
    pub translation: Vector3<f64>,
    //pub scale: Vector3<f32>,
}

impl Transform{
    pub fn as_matrix(&self) -> Matrix4<f64>{
        Similarity3::new(self.translation,self.rotation.rotation(),1.0)
            .to_homogeneous()
    }

    pub fn translate(&mut self,Vector3<f64>){
        self.translation += Vector3::new(x,y,z);
    }
}

impl Default for Transform{
    fn default() -> Self{
        Transform{
            rotation: UnitQuaternion::new_with_euler_angles(0.0,0.0,0.0),
            translation: Vector3::new(0.0,0.0,0.0),
        }
    }
}
