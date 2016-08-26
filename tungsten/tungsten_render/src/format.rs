extern crate nalgebra;
use self::nalgebra::{Perspective3,UnitQuaternion,Vector3,Matrix4,ToHomogeneous,Similarity3,Rotation};
use super::tungsten_asset::{Container,Mesh};

#[derive(Clone)]
pub struct StaticRenderObject{
    pub mesh: Container<Mesh>,
    pub transform: Transform,
}

pub struct Camera{
    pub perspective: Perspective3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub translation: Vector3<f32>,
}

impl Camera{
    pub fn as_matrix(&self) -> Matrix4<f32>{
        let mat = Similarity3::new(self.translation,self.rotation.rotation(),1.0)
            .to_homogeneous();
        mat * self.perspective.to_matrix()
    }
}

#[derive(Clone,Debug)]
pub struct Transform{
    pub rotation: UnitQuaternion<f32>,
    pub translation: Vector3<f32>,
    //pub scale: Vector3<f32>,
}

impl Transform{
    pub fn as_matrix(&self) -> Matrix4<f32>{
        Similarity3::new(self.translation,self.rotation.rotation(),1.0)
            .to_homogeneous()
    }

    pub fn translate(&mut self,x: f32,y: f32, z: f32){
        self.translation += Vector3::new(x,y,z);
    }
}

impl Default for Transform{
    fn default() -> Self{
        Transform{
            rotation: UnitQuaternion::from_euler_angles(0.0,0.0,0.0),
            translation: Vector3::new(0.0,0.0,0.0),
        }
    }
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
            rotation: UnitQuaternion::from_euler_angles(0.0,0.0,1.0),
            translation: Vector3::new(0.0,0.0,0.0),
        }
    }
}
