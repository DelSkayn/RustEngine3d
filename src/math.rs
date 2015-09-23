use std::ops::{
    Div,
    Sub,
    Mul,
    Add,
};

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

#[derive(Copy,Clone,Debug)]
pub struct Vector2<T>
    where T: Copy + Clone + Debug{
        vec: [T; 2],
}

impl<T> Vector2<T>
    where T: Default + Copy + Debug{ 
    pub fn new() -> Self{
        Vector2{
            vec: [T::default(); 2], 
        }
    }

    pub fn form_array(array: [T; 2]) -> Self{
        Vector2{
            vec: array,
        }
    }

    pub fn form_coords(x: T,y: T) -> Self{
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
    where T:Add<Output = T> + Mul<Output = T> + Copy + Debug{
        pub fn dot(&self,other: &Vector2<T>) -> T{
            self.vec[0] * other.vec[0] +
                self.vec[1] * self.vec[1]
        }
}

impl Vector2<f32>{
    pub fn from_angle(angle: f32) -> Self{
        Vector2{
            vec: [angle.cos(),angle.sin()],
        }
    }

    pub fn from_angle_degrees(angle: f32) -> Self{
        Self::from_angle(angle / 360.0 * fconsts::PI)
    }

    pub fn normalize(&mut self){
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
    }

    pub fn length(&self) -> f32{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]).sqrt()
    }

    pub fn length_2(&self) -> f32{
        self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
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

    pub fn normalize(&mut self){
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
    }

    pub fn length(&self) -> f64{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]).sqrt()
    }

    pub fn length_2(&self) -> f64{
        self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
    }
}

impl<T> Add for Vector2<T>
    where T: Add<Output = T> + Copy + Debug{//That is amazing that you can specify Output
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T>{
        Vector2{
            vec: [self.vec[0] + other.vec[0],
                self.vec[1] + other.vec[1]],
        }
    }
}

impl<'a,T> Add for &'a Vector2<T>
    where T: Add<Output = T> + Copy + Debug{
    type Output = Vector2<T>;

    fn add(self, other: &'a Vector2<T>) -> Vector2<T>{
        Vector2{
            vec: [self.vec[0] + other.vec[0],
                self.vec[1] + other.vec[1]],
        }
    }
}

impl<T> Sub for Vector2<T>
    where T: Sub<Output = T> + Copy + Debug{//That is amazing
    type Output = Vector2<T>;

    fn sub(self, other: Vector2<T>) -> Vector2<T>{
        Vector2{
            vec: [self.vec[0] - other.vec[0],
                self.vec[1] - other.vec[1]],
        }
    }
}

impl<T> Mul<T> for Vector2<T>
    where T: Mul<Output = T> + Copy + Debug{
    type Output = Vector2<T>;

    fn mul(self, other: T) -> Vector2<T>{
        Vector2{
            vec: [self.vec[0] * other,
                self.vec[1] * other],
        }
    }
}

impl<T> Div<T> for Vector2<T>
    where T: Div<Output = T> + Copy + Debug{//That is amazing
    type Output = Vector2<T>;

    fn div(self, other: T) -> Vector2<T>{
        Vector2{
            vec: [self.vec[0] / other,
                self.vec[1] / other],
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Vector3<T>
    where T: Copy + Debug{
    
        vec: [T; 3],
}

impl<T> Vector3<T>
    where T: Default + Copy + Debug{ 
    pub fn new() -> Self{
        Vector3{
            vec: [T::default(); 3], 
        }
    }

    pub fn form_array(array: [T; 3]) -> Self{
        Vector3{
            vec: array,
        }
    }

    pub fn form_coords(x: T,y: T,z: T) -> Self{
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
    pub fn normalize(&mut self){
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
        self.vec[2] /= length;
    }

    pub fn length(&self) -> f32{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
         + self.vec[2] * self.vec[2]).sqrt()
    }
}

impl Vector3<f64>{
    pub fn normalize(&mut self){
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
        self.vec[2] /= length;
    }

    pub fn length(&self) -> f64{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
         + self.vec[2] * self.vec[2]).sqrt()
    }
}

impl<T> Vector3<T>
    where T:Mul<Output = T> + Add<Output = T> + Copy + Debug{
        pub fn dot(&self,other: &Vector2<T>) -> T{
            self.vec[0] * other.vec[0] +
                self.vec[1] * self.vec[1] +
                self.vec[2] * self.vec[2]
        }
}

impl<T> Add for Vector3<T>
    where T: Add<Output = T> + Copy + Debug{//That is amazing that you can specify Output
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
    where T: Add<Output = T> + Copy + Debug{
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
    where T: Sub<Output = T> + Copy + Debug{//That is amazing
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
    where T: Mul<Output = T> + Copy + Debug{//That is amazing
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
    where T: Div<Output = T> + Copy + Debug{//That is amazing
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

#[derive(Copy,Clone,Debug)]
pub struct Vector4<T>
    where T: Debug + Copy{
        vec: [T; 4],
}

impl<T> Vector4<T>
    where T: Default + Copy + Debug{ 
    pub fn new() -> Self{
        Vector4{
            vec: [T::default(); 4], 
        }
    }

    pub fn form_array(array: [T; 4]) -> Self{
        Vector4{
            vec: array,
        }
    }

    pub fn form_coords(w: T,x: T,y: T,z: T) -> Self{
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
    where T:Mul<Output = T> + Add<Output = T> + Copy + Debug{
        pub fn dot(&self,other: &Vector2<T>) -> T{
            self.vec[0] * other.vec[0] +
                self.vec[1] * self.vec[1] +
                self.vec[2] * self.vec[2] +
                self.vec[3] * self.vec[3]
        }
}

impl Vector4<f32>{
    pub fn normalize(&mut self){
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
        self.vec[2] /= length;
    }

    pub fn length(&self) -> f32{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
         + self.vec[2] * self.vec[2]).sqrt()
    }
}

impl Vector4<f64>{
    pub fn normalize(&mut self){
        let length = self.length();
        self.vec[0] /= length;
        self.vec[1] /= length;
        self.vec[2] /= length;
    }

    pub fn length(&self) -> f64{
        (self.vec[0] * self.vec[0] 
         + self.vec[1] * self.vec[1]
         + self.vec[2] * self.vec[2]).sqrt()
    }
}

impl<T> Add for Vector4<T>
    where T: Add<Output = T> + Copy + Debug{//That is amazing that you can specify Output
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
    where T: Add<Output = T> + Copy + Debug{
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
    where T: Sub<Output = T> + Copy + Debug{//That is amazing
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
    where T: Mul<Output = T> + Copy + Debug{//That is amazing
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
    where T: Div<Output = T> + Copy + Debug{//That is amazing
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

#[derive(Copy,Clone,Debug)]
pub struct Matrix3<T>
    where T: Copy + Debug{
    mat: [[T; 3]; 3],
}

impl<T> Matrix3<T>
    where T: Default + Copy + Debug{
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

impl Mul for Matrix3<T>
where T: Add<Output = T> + Mul<Output = T>{
    type Output = Self;
    fn mul(self,other: Self){
        Matrix3{
            mat: [
                
        }
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

#[derive(Copy,Clone,Debug)]
pub struct Matrix4<T>
where T: Copy + Debug{
    mat: [[T; 4]; 4],
}

impl<T> Matrix4<T>
where T: Default + Copy + Debug{
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
