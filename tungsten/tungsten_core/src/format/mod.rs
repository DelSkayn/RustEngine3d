
extern crate nalgebra;

pub type Float = f64;

pub type Vector3 = nalgebra::Vector3<Float>;
pub type Vector4 = nalgebra::Vector4<Float>;
pub type Point3 = nalgebra::Point3<Float>;
pub type Matrix4 = nalgebra::Matrix4<Float>;
pub type Quaternion = nalgebra::UnitQuaternion<Float>;
pub type Perspective = nalgebra::Perspective3<Float>;


mod transform;

pub use self::transform::Transform;

