extern crate cgmath;

use self::cgmath::{Vector2,Vector3,Vector4,Point2,Point3,Matrix2,Matrix3,Matrix4};

pub trait Cast<T>{
    fn cast(t: T) -> Self;
}

impl Cast<Point2<f64>> for Point2<f32>{
    fn cast(p: Point2<f64>) -> Self{
        Point2{
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}

impl Cast<Point3<f64>> for Point3<f32>{
    fn cast(p: Point3<f64>) -> Self{
        Point3{
            x: p.x as f32,
            y: p.y as f32,
            z: p.z as f32,
        }
    }
}
impl Cast<Vector2<f64>> for Vector2<f32>{
    fn cast(vec: Vector2<f64>) -> Self{
        Vector2{
            x: vec.x as f32,
            y: vec.y as f32,
        }
    }
}


impl Cast<Vector3<f64>> for Vector3<f32>{
    fn cast(vec: Vector3<f64>) -> Self{
        Vector3{
            x: vec.x as f32,
            y: vec.y as f32,
            z: vec.z as f32,
        }
    }
}

impl Cast<Vector4<f64>> for Vector4<f32>{
    fn cast(vec: Vector4<f64>) -> Self{
        Vector4{
            x: vec.x as f32,
            y: vec.y as f32,
            z: vec.z as f32,
            w: vec.w as f32,
        }
    }
}

impl Cast<Matrix2<f64>> for Matrix2<f32>{
    fn cast(mat: Matrix2<f64>) -> Self{
        Matrix2{
            x: Cast::cast(mat.x),
            y: Cast::cast(mat.y),
        }
    }
}

impl Cast<Matrix3<f64>> for Matrix3<f32>{
    fn cast(mat: Matrix3<f64>) -> Self{
        Matrix3{
            x: Cast::cast(mat.x),
            y: Cast::cast(mat.y),
            z: Cast::cast(mat.z),
        }
    }
}

impl Cast<Matrix4<f64>> for Matrix4<f32>{
    fn cast(mat: Matrix4<f64>) -> Self{
        Matrix4{
            x: Cast::cast(mat.x),
            y: Cast::cast(mat.y),
            z: Cast::cast(mat.z),
            w: Cast::cast(mat.w),
        }
    }
}
