
use super::{Renderer,Error};

pub struct Ogl;

impl Renderer for Ogl{
}

impl Ogl{
    pub fn new() -> Result<Self,Error>{
        Ok(Ogl)
    }
}
