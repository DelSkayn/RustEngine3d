use std::ops::{
    Div,
    Sub,
    Mul,
    Add, };

use std::default::Default;
use std::fmt::Debug;

use std::f32::consts as fconsts;
use std::f64::consts as dconsts;

pub type Vector2f = Vector2<f32>;
pub type Vector3f = Vector3<f32>;
pub type Vector4f = Vector4<f32>;

pub type Vector2d = Vector2<f64>;
pub type Vector3d = Vector3<f64>;
pub type Vector4d = Vector4<f64>;

pub type Matrix3f = Matrix3<f32>;
pub type Matrix4f = Matrix4<f32>;

pub type Matrix3d = Matrix3<f64>;
pub type Matrix4d = Matrix4<f64>;

pub type Quatf = Quat<f32>;
pub type Quatd = Quat<f64>;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Vector2<T>
    where T: Copy + Clone + Debug  + PartialEq{
        vec: [T; 2],
}

impl<T> Vector2<T>
    where T: Default + Copy + Debug  + PartialEq{ 
    pub fn new() -> Self{
        Vector2{
            vec: [T::default(); 2], 
        }
    }

    pub fn from_array(array: [T; 2]) -> Self{
        Vector2{
            vec: array,
        }
    }

    pub fn from_coords(x: T,y: T) -> Self{
        Vector2{
            vec: [x,y],
        }
    }

    pub fn from_tuple(tup: (T,T)) -> Self{
        Vector2{
            vec: [tup.0,tup.1],
        }
    }

}

impl<T> Vector2<T>
    where T:Add<Output = T> + Mul<Output = T> + Copy + Debug  + PartialEq{
        pub fn dot(&self,other: &Vector2<T>) -> T{
            self.vec[0] * other.vec[0] +
                self.vec[1] * self.vec[1]
        }
}

impl Vector2<f32>{
    pub fn from_angle(angle: f32) -> Self{
        Vector2{
            vec: [angle.sin(),angle.cos()],
        }
    }

    pub fn from_angle_degrees(angle: f32) -> Self{
        Self::from_angle(angle / 180.0 * fconsts::PI)
    }

    pub fn normalize(mut self) -> Self{
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
        self
    }

    pub fn length(&self) -> f32{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]).sqrt()
    }

    pub fn length_2(&self) -> f32{
        self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
    }

    pub fn cross(&self,other: &Self) -> f32{
        self.vec[0] * other.vec[1] - self.vec[1] * other.vec[0]
    }
}

impl Vector2<f64>{
    pub fn from_angle(angle: f64) -> Self{
        Vector2{
            vec: [angle.cos(),angle.sin()],
        }
    }

    pub fn from_angle_degrees(angle: f64) -> Self{
        Self::from_angle(angle / 360.0 * dconsts::PI)
    }

    pub fn normalize(mut self) -> Self{
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
        self
    }

    pub fn length(&self) -> f64{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]).sqrt()
    }

    pub fn length_2(&self) -> f64{
        self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
    }

    pub fn cross(&self,other: &Self) -> f64{
        self.vec[0] * other.vec[1] - self.vec[1] * other.vec[0]
    }
}

impl<T> Add for Vector2<T>
    where T: Add<Output = T> + Copy + Debug  + PartialEq {//That is amazing that you can specify Output
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T>{
        Vector2{
            vec: [self.vec[0] + other.vec[0],
                self.vec[1] + other.vec[1]],
        }
    }
}

impl<'a,T> Add for &'a Vector2<T>
    where T: Add<Output = T> + Copy + Debug  + PartialEq {
    type Output = Vector2<T>;

    fn add(self, other: &'a Vector2<T>) -> Vector2<T>{
        Vector2{
            vec: [self.vec[0] + other.vec[0],
                self.vec[1] + other.vec[1]],
        }
    }
}

impl<T> Sub for Vector2<T>
    where T: Sub<Output = T> + Copy + Debug  + PartialEq {//That is amazing
    type Output = Vector2<T>;

    fn sub(self, other: Vector2<T>) -> Vector2<T>{
        Vector2{
            vec: [self.vec[0] - other.vec[0],
                self.vec[1] - other.vec[1]],
        }
    }
}

impl<T> Mul<T> for Vector2<T>
    where T: Mul<Output = T> + Copy + Debug  + PartialEq {
    type Output = Vector2<T>;

    fn mul(self, other: T) -> Vector2<T>{
        Vector2{
            vec: [self.vec[0] * other,
                self.vec[1] * other],
        }
    }
}

impl<T> Div<T> for Vector2<T>
    where T: Div<Output = T> + Copy + Debug  + PartialEq {//That is amazing
    type Output = Vector2<T>;

    fn div(self, other: T) -> Vector2<T>{
        Vector2{
            vec: [self.vec[0] / other,
                self.vec[1] / other],
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Vector3<T>
    where T: Copy + Debug  + PartialEq {
    
        vec: [T; 3],
}

impl<T> Vector3<T>
    where T: Default + Copy + Debug  + PartialEq { 
    pub fn new() -> Self{
        Vector3{
            vec: [T::default(); 3], 
        }
    }

    pub fn from_array(array: [T; 3]) -> Self{
        Vector3{
            vec: array,
        }
    }

    pub fn from_coords(x: T,y: T,z: T) -> Self{
        Vector3{
            vec: [x,y,z],
        }
    }

    pub fn from_tuple(tup: (T,T,T)) -> Self{
        Vector3{
            vec: [tup.0,tup.1,tup.2],
        }
    }
}

impl Vector3<f32>{
    pub fn normalize(mut self) -> Self{
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
        self.vec[2] /= length;
        self
    }

    pub fn length(&self) -> f32{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
         + self.vec[2] * self.vec[2]).sqrt()
    }

    pub fn cross(mut self, other: &Self) -> Self{
        self.vec[0] = self.vec[1] * other.vec[2] - self.vec[2] * other.vec[1];
        self.vec[1] = self.vec[2] * other.vec[0] - self.vec[0] * other.vec[2];
        self.vec[2] = self.vec[0] * other.vec[1] - self.vec[1] * other.vec[0];
        self
    }
}

impl Vector3<f64>{
    pub fn normalize(mut self) -> Self{
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
        self.vec[2] /= length;
        self
    }

    pub fn length(&self) -> f64{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
         + self.vec[2] * self.vec[2]).sqrt()
    }

    pub fn cross(mut self, other: &Self) -> Self{
        self.vec[0] = self.vec[1] * other.vec[2] - self.vec[2] * other.vec[1];
        self.vec[1] = self.vec[2] * other.vec[0] - self.vec[0] * other.vec[2];
        self.vec[2] = self.vec[0] * other.vec[1] - self.vec[1] * other.vec[0];
        self
    }
}

impl<T> Vector3<T>
    where T:Mul<Output = T> + Add<Output = T> + Copy + Debug  + PartialEq {
        pub fn dot(&self,other: &Vector2<T>) -> T{
            self.vec[0] * other.vec[0] +
                self.vec[1] * self.vec[1] +
                self.vec[2] * self.vec[2]
        }
}

impl<T> Add for Vector3<T>
    where T: Add<Output = T> + Copy + Debug  + PartialEq {//That is amazing that you can specify Output
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Vector3<T>{
        Vector3{
            vec: [
                self.vec[0] + other.vec[0],
                self.vec[1] + other.vec[1],
                self.vec[2] + other.vec[2]
            ],
        }
    }
}

impl<'a,T> Add for &'a Vector3<T>
    where T: Add<Output = T> + Copy + Debug  + PartialEq {
    type Output = Vector3<T>;

    fn add(self, other: &'a Vector3<T>) -> Vector3<T>{
        Vector3{
            vec: [
                self.vec[0] + other.vec[0],
                self.vec[1] + other.vec[1],
                self.vec[2] + other.vec[2]
            ],
        }
    }
}

impl<T> Sub for Vector3<T>
    where T: Sub<Output = T> + Copy + Debug  + PartialEq {//That is amazing
    type Output = Vector3<T>;

    fn sub(self, other: Vector3<T>) -> Vector3<T>{
        Vector3{
            vec: [
                self.vec[0] - other.vec[0],
                self.vec[1] - other.vec[1],
                self.vec[2] - other.vec[2]
            ],
        }
    }
}

impl<T> Mul<T> for Vector3<T>
    where T: Mul<Output = T> + Copy + Debug  + PartialEq {//That is amazing
    type Output = Vector3<T>;

    fn mul(self, other: T) -> Vector3<T>{
        Vector3{
            vec: [
                self.vec[0] * other,
                self.vec[1] * other,
                self.vec[2] * other
            ],
        }
    }
}

impl<T> Div<T> for Vector3<T>
    where T: Div<Output = T> + Copy + Debug  + PartialEq {//That is amazing
    type Output = Vector3<T>;

    fn div(self, other: T) -> Vector3<T>{
        Vector3{
            vec: [
                self.vec[0] / other,
                self.vec[1] / other,
                self.vec[2] / other
            ],
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Vector4<T>
    where T: Debug + Copy{
        vec: [T; 4],
}

impl<T> Vector4<T>
    where T: Default + Copy + Debug  + PartialEq { 
    pub fn new() -> Self{
        Vector4{
            vec: [T::default(); 4], 
        }
    }

    pub fn from_array(array: [T; 4]) -> Self{
        Vector4{
            vec: array,
        }
    }

    pub fn from_coords(w: T,x: T,y: T,z: T) -> Self{
        Vector4{
            vec: [w,x,y,z],
        }
    }

    pub fn from_tuple(tup: (T,T,T,T)) -> Self{
        Vector4{
            vec: [tup.0,tup.1,tup.2,tup.3],
        }
    }
}

impl<T> Vector4<T>
    where T:Mul<Output = T> + Add<Output = T> + Copy + Debug  + PartialEq {
        pub fn dot(&self,other: &Vector2<T>) -> T{
            self.vec[0] * other.vec[0] +
                self.vec[1] * self.vec[1] +
                self.vec[2] * self.vec[2] +
                self.vec[3] * self.vec[3]
        }
}

impl Vector4<f32>{
    pub fn normalize(mut self) ->Self{
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
        self.vec[2] /= length;
        self.vec[3] /= length;
        self
    }

    pub fn length(&self) -> f32{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
         + self.vec[2] * self.vec[2]).sqrt()
    }
}

impl Vector4<f64>{
    pub fn normalize(mut self) ->Self{
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
        self.vec[2] /= length;
        self.vec[3] /= length;
        self
    }

    pub fn length(&self) -> f64{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
         + self.vec[2] * self.vec[2]).sqrt()
    }
}

impl<T> Add for Vector4<T>
    where T: Add<Output = T> + Copy + Debug  + PartialEq {//That is amazing that you can specify Output
    type Output = Vector4<T>;

    fn add(self, other: Vector4<T>) -> Vector4<T>{
        Vector4{
            vec: [
                self.vec[0] + other.vec[0],
                self.vec[1] + other.vec[1],
                self.vec[2] + other.vec[2],
                self.vec[3] + other.vec[3]
            ],
        }
    }
}

impl<'a,T> Add for &'a Vector4<T>
    where T: Add<Output = T> + Copy + Debug  + PartialEq {
    type Output = Vector4<T>;

    fn add(self, other: &'a Vector4<T>) -> Vector4<T>{
        Vector4{
            vec: [
                self.vec[0] + other.vec[0],
                self.vec[1] + other.vec[1],
                self.vec[2] + other.vec[2],
                self.vec[3] + other.vec[3]
            ],
        }
    }
}

impl<T> Sub for Vector4<T>
    where T: Sub<Output = T> + Copy + Debug  + PartialEq {//That is amazing
    type Output = Vector4<T>;

    fn sub(self, other: Vector4<T>) -> Vector4<T>{
        Vector4{
            vec: [
                self.vec[0] - other.vec[0],
                self.vec[1] - other.vec[1],
                self.vec[2] - other.vec[2],
                self.vec[3] - other.vec[3]
            ],
        }
    }
}

impl<T> Mul<T> for Vector4<T>
    where T: Mul<Output = T> + Copy + Debug  + PartialEq {//That is amazing
    type Output = Vector4<T>;

    fn mul(self, other: T) -> Vector4<T>{
        Vector4{
            vec: [
                self.vec[0] * other,
                self.vec[1] * other,
                self.vec[2] * other,
                self.vec[3] * other
            ],
        }
    }
}

impl<T> Div<T> for Vector4<T>
    where T: Div<Output = T> + Copy + Debug  + PartialEq {//That is amazing
    type Output = Vector4<T>;

    fn div(self, other: T) -> Vector4<T>{
        Vector4{
            vec: [
                self.vec[0] / other,
                self.vec[1] / other,
                self.vec[2] / other,
                self.vec[3] / other
            ],
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Matrix3<T>
    where T: Copy + Debug  + PartialEq {
    mat: [[T; 3]; 3],
}

impl<T> Matrix3<T>
    where T: Default + Copy + Debug  + PartialEq {
    pub fn new() -> Self{
        Matrix3{
            mat: [[T::default(); 3]; 3],
        }
    }

    pub fn from_array(array: [[T; 3]; 3]) -> Self{
        Matrix3{
            mat: array,
        }
    }

    pub fn from_vector(fir: Vector3<T>,sec: Vector3<T>,thi: Vector3<T>) -> Self{
        Matrix3{
            mat: [
                    fir.vec,
                    sec.vec,
                    thi.vec
                ],
        }
    }
}

impl<T> Mul for Matrix3<T>
where T: Add<Output = T> + Mul<Output = T> + Copy + Debug  + PartialEq  + Default{
    type Output = Self;
    fn mul(self,other: Self) -> Self{
        let mut res = Matrix3::<T>::new();
        for i in 0..3{
            for j in 0..3{
                for k in 0..3{
                    res.mat[i][j] = res.mat[i][j] + self.mat[k][j] * other.mat[i][k];
                }
            }
        }
        res
    }
}

impl Matrix3<f32>{
    pub fn identity() -> Self{
        Matrix3{
            mat: [
                [1.0,0.0,0.0],
                [0.0,1.0,0.0],
                [0.0,0.0,1.0]
            ],
        }
    }
}

impl Matrix3<f64>{
    pub fn identity() -> Self{
        Matrix3{
            mat: [
                [1.0,0.0,0.0],
                [0.0,1.0,0.0],
                [0.0,0.0,1.0]
            ],
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Matrix4<T>
where T: Copy + Debug  + PartialEq {
    mat: [[T; 4]; 4],
}

impl<T> Matrix4<T>
where T: Default + Copy + Debug  + PartialEq {
    pub fn new() -> Self{
        Matrix4{
            mat: [[T::default(); 4]; 4],
        }
    }

    pub fn from_array(array: [[T; 4]; 4]) -> Self{
        Matrix4{
            mat: array,
        }
    }


    pub fn from_vector(fir: Vector4<T>,sec: Vector4<T>,thi: Vector4<T>,fou: Vector4<T>) -> Self{
        Matrix4{
            mat: [
                fir.vec,
                sec.vec,
                thi.vec,
                fou.vec
            ],
        }
    }
}

impl Matrix4<f32>{
    pub fn as_rotation(forward: Vector3<f32>,up: Vector3<f32>) -> Self{
        let n = forward.normalize();
        let u = up.normalize().cross(&n);
        let v = n.clone().cross(&u);

        Self::as_rotation_full(n,v,u)
    }

    pub fn as_rotation_full(n: Vector3<f32>,v: Vector3<f32>,u: Vector3<f32>) -> Self{
        Matrix4{
            mat: [
                [u.vec[0],v.vec[0],n.vec[0],0.0],
                [u.vec[1],v.vec[1],n.vec[1],0.0],
                [u.vec[2],v.vec[2],n.vec[2],0.0],
                [0.0,0.0,0.0,1.0]
            ]
        }
    }

    pub fn as_perspective(fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self{
        let z_range = z_far-z_near;
        let tan_half_fov = (fov/2.0).tan();

        Matrix4{
            mat: [
                [1.0/(tan_half_fov * aspect_ratio), 0.0, 0.0,0.0],
                [0.0,1.0/tan_half_fov,0.0,0.0],
                [0.0,0.0,(-z_near - z_far)/z_range,-1.0],
                [0.0,0.0,2.0*-z_far*z_near/z_range,0.0]
            ]
        }
    }
}

impl Matrix4<f64>{
    pub fn as_rotation(forward: Vector3<f64>,up: Vector3<f64>) -> Self{
        let n = forward.normalize();
        let u = up.normalize().cross(&n);
        let v = n.clone().cross(&u);

        Self::as_rotation_full(n,v,u)
    }

    pub fn as_rotation_full(n: Vector3<f64>,v: Vector3<f64>,u: Vector3<f64>) -> Self{
        Matrix4{
            mat: [
                [u.vec[0],v.vec[0],n.vec[0],0.0],
                [u.vec[1],v.vec[1],n.vec[1],0.0],
                [u.vec[2],v.vec[2],n.vec[2],0.0],
                [0.0,0.0,0.0,1.0]
            ]
        }
    }

    pub fn as_perspective(fov: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Self{
        let z_range = z_far-z_near;
        let tan_half_fov = (fov/2.0).tan();

        Matrix4{
            mat: [
                [1.0/(tan_half_fov * aspect_ratio), 0.0, 0.0,0.0],
                [0.0,1.0/tan_half_fov,0.0,0.0],
                [0.0,0.0,(-z_near - z_far)/z_range,-1.0],
                [0.0,0.0,2.0*-z_far*z_near/z_range,0.0]
            ]
        }
    }
}

impl<T> Mul for Matrix4<T>
where T: Add<Output = T> + Mul<Output = T> + Copy + Debug  + PartialEq  + Default{
    type Output = Self;
    fn mul(self,other: Self) -> Self{
        let mut res = Matrix4::<T>::new();
        for i in 0..4{
            for j in 0..4{
                for k in 0..4{
                    res.mat[i][j] = res.mat[i][j] + (self.mat[k][j] * other.mat[i][k]);
                }
            }
        }
        res
    }
}

impl Matrix4<f32>{
    pub fn identity() -> Self{
        Matrix4{
            mat: [
                [1.0,0.0,0.0,0.0],
                [0.0,1.0,0.0,0.0],
                [0.0,0.0,1.0,0.0],
                [0.0,0.0,0.0,1.0],
            ],
        }
    }
}

impl Matrix4<f64>{
    pub fn identity() -> Self{
        Matrix4{
            mat: [
                [1.0,0.0,0.0,0.0],
                [0.0,1.0,0.0,0.0],
                [0.0,0.0,1.0,0.0],
                [0.0,0.0,0.0,1.0],
            ],
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Quat<T>
    where T: Copy + Clone + Debug + PartialEq{
    quat: [T; 4],
}

impl<T> Quat<T>
    where T: Copy + Clone + Debug + PartialEq{
        pub fn from_coords(w: T,x: T,y: T,z: T) -> Self{
            Quat{
                quat:[w,x,y,z],
            }
        }

        pub fn from_array(array: [T; 4]) -> Self{
            Quat{
                quat: array,
            }
        }

        pub fn from_vector(vec: Vector4<T>) -> Self{
            Quat{
                quat: vec.vec,
            }
        }
}

impl Quat<f32>{
        pub fn new() -> Self{
            Quat{
                quat: [0.0,0.0,0.0,1.0],
            }
        }

        pub fn from_angle(axis: Vector3<f32>, angle: f32) -> Self{
            let angle_2 = angle/2.0;
            let sin = angle_2.sin();

            Quat{
                quat: [axis.vec[0] * sin,
                    axis.vec[1] * sin,
                    axis.vec[2] * sin,
                    angle_2.cos()],
            }
        }

}

impl Quat<f64>{
        pub fn new() -> Self{
            Quat{
                quat: [0.0,0.0,0.0,1.0],
            }
        }

        pub fn from_angle(axis: Vector3<f64>, angle: f64) -> Self{
            let angle_2 = angle/2.0;
            let sin = angle_2.sin();

            Quat{
                quat: [axis.vec[0] * sin,
                    axis.vec[1] * sin,
                    axis.vec[2] * sin,
                    angle_2.cos()],
            }
        }
}

impl<T> Mul for Quat<T>
    where T: Mul<Output=T> + Sub<Output=T> + Add<Output=T> + Copy + Debug + PartialEq{
        type Output = Self;
        fn mul(self,other: Quat<T>) -> Self{
            Quat{
                quat: [
                    self.quat[0] * other.quat[0] - self.quat[1] * other.quat[1] - self.quat[2] * other.quat[2] - self.quat[3] * other.quat[2],
                    self.quat[1] * other.quat[0] + self.quat[0] * other.quat[1] + self.quat[2] * other.quat[3] - self.quat[3] * other.quat[2],
                    self.quat[2] * other.quat[0] + self.quat[0] * other.quat[2] + self.quat[3] * other.quat[1] - self.quat[1] * other.quat[3],
                    self.quat[3] * other.quat[0] + self.quat[0] * other.quat[3] + self.quat[1] * other.quat[2] - self.quat[2] * other.quat[1]
                ],
            }
        }
}

impl<T> Mul<Vector3<T>> for Quat<T>
    where T: Mul<Output=T> + Sub<Output=T> + Add<Output=T> + Copy + Debug + PartialEq{
        type Output = Self;
        fn mul(self,other: Vector3<T>) -> Self{
            Quat{
                quat: [
                    self.quat[1] * other.vec[0] - self.quat[2] * other.vec[1] - self.quat[3] * other.vec[2],
                    self.quat[0] * other.vec[0] + self.quat[2] * other.vec[2] - self.quat[3] * other.vec[1],
                    self.quat[0] * other.vec[1] + self.quat[3] * other.vec[0] - self.quat[1] * other.vec[2],
                    self.quat[0] * other.vec[2] + self.quat[1] * other.vec[1] - self.quat[2] * other.vec[0]
                ],
            }
        }
}

#[cfg(test)]
mod test{
    #![allow(dead_code)]
    use super::*;
    use std::f32::consts;
    #[test]
    fn test(){
        let vec = Vector2f::new();
        let vec2 = Vector2f::from_array([3.4,5.6]);
        let vec3 = Vector2f::from_angle(consts::PI/2.0);
        let vec4 = Vector2f::from_angle_degrees(90.0);
        let add = vec + vec2 + vec3 + vec4;
        println!("{:?}",vec);
        println!("{:?}",vec2);
        println!("{:?}",vec3);
        println!("{:?}",vec4);
        println!("{:?}", add);
        assert!(Vector2f::from_array([5.4,5.6]) == add);

        let mat  = Matrix4::from_array([
                       [3.0,2.0,5.0,7.0],
                       [6.0,7.0,4.0,3.0],
                       [9.0,5.0,3.0,5.0],
                       [9.0,5.0,3.0,5.0]
        ]);

        let mat2 = Matrix4::from_array([
                       [1.0,2.0,3.0,5.0],
                       [5.0,3.0,6.0,5.0],
                       [2.0,4.0,7.0,2.0],
                       [9.0,5.0,7.0,1.0]
        ]);

        let matmul = mat * mat2;
        println!("{:?}",mat);
        println!("{:?}",mat2);
        println!("{:?}",matmul);
        assert!(Matrix4::from_array([
                                   [87.0,56.0,37.0,53.0],
                                   [132.0,86.0,70.0,99.0],
                                   [111.0,77.0,53.0,71.0],
                                   [129.0,93.0,89.0,118.0]
        ]) == matmul);
    }

}
