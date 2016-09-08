use super::cgmath::*;


static ID_USIZE_MAP: u64 = 0x00ffffffffffffff;
static GEN_USIZE_MAP: u64 = 0xff00000000000000;

use std::collections::VecDeque;

/// Struct containing data about a render object.
/// This struct will be the external interface for changing renderobject data.
#[derive(Clone,Copy)]
pub struct RenderObject {
    /// position, scale and rotation of the renderobject.
    pub transform: Decomposed<Vector3<f64>, Quaternion<f64>>,
    /// wether the render object should be rendered
    pub hidden: bool,
}

/// Struct containing data about a object which needs to be rendered.
/// This is the struct that the backends will recieve.
#[derive(Clone,Copy)]
pub struct RenderData {
    pub id: GenId,
    pub transform: Matrix4<f32>,
}


/// A Id with a generation.
/// The generation is to indicate wether the a certain id has already been used.
/// This is done to preserve indecies.
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct GenId(u64);

/// An generator for creating and freeing GenId's
pub struct Generator {
    next: usize,
    free: VecDeque<GenId>,
}

impl GenId {
    /// Create a new GenId from a object and a index.
    /// Index must be smaller then `1 << 55`
    fn new(gen: u8, index: usize) -> GenId {
        assert!(index <= ID_USIZE_MAP as usize);
        GenId((index | ((gen as usize) << 54)) as u64)
    }

    /// Returns the id.
    pub fn id(&self) -> usize {
        (self.0 & ID_USIZE_MAP) as usize
    }

    /// Returns the generation.
    pub fn gen(&self) -> u8 {
        (self.0 & GEN_USIZE_MAP >> 56) as u8
    }
}

impl Generator {
    /// Create a new  Generator object.
    pub fn new() -> Self {
        Generator {
            next: 0,
            free: VecDeque::new(),
        }
    }

    /// Returns a new GenId, the index might be used already but the
    /// generation will then not be the same.
    /// Warning: GenId can only differ between 256 different id's of the same index.
    /// After 256 of ids with same index being freed the a id which
    /// was already given might be returned.
    pub fn next(&mut self) -> GenId {
        if let Some(x) = self.free.pop_back() {
            return x;
        }
        let next = self.next;
        self.next += 1;
        GenId::new(0, next)
    }

    /// Frees a GenId so it might be used again.
    pub fn free(&mut self, id: GenId) {
        let new_id = id.id();
        let mut gen = id.gen();
        gen = gen.wrapping_add(1);
        self.free.push_front(GenId::new(gen, new_id));
    }
}
