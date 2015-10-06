extern crate image;

use image::{
    DynamicImage,
    GenericImage
};

use super::resource::Resource;

struct Texture{
    width: i32,
    heigth: i32,
    text: Vec<u8>,
}
