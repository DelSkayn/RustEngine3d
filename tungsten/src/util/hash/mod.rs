mod xx_hash;

use self::xx_hash::XXHasher;

use std::hash::Hasher;
use std::hash::BuildHasher;

use std::mem;


pub struct FastHashBuilder;

impl FastHashBuilder{
    pub fn new() -> Self{
        FastHashBuilder
    }
}

impl BuildHasher for FastHashBuilder{
    type Hasher = XXHasher;

    fn build_hasher(&self) -> Self::Hasher{
        Self::Hasher::new()
    }
}

pub struct NoHasher{
    res: u64,
}

impl NoHasher{
    fn new() -> Self{
        NoHasher{
            res: 0,
        }
    }
}

impl Hasher for NoHasher{

    fn write(&mut self,value: &[u8]){
        assert!(value.len() == mem::size_of::<u64>());
        unsafe{
            let p: *const u64 = mem::transmute(value.as_ptr());
            self.res = *p;
        }
    }

    fn finish(&self) -> u64{
        self.res
    }
}

pub struct NoHashBuilder;

impl NoHashBuilder{
    pub fn new() -> Self{
        NoHashBuilder
    }
}

impl BuildHasher for NoHashBuilder{
    type Hasher = NoHasher;

    fn build_hasher(&self) -> Self::Hasher{
        Self::Hasher::new()
    }

}

pub type HashAlgo = XXHasher;

