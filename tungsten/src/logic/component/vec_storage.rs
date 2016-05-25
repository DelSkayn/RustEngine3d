
use std::mem;

use super::ComponentStorage;
use super::Component;

struct BitMap(Vec<u8>);

impl BitMap {
    fn new() -> Self {
        BitMap(Vec::new())
    }

    fn set(&mut self, i: usize) {
        let index = i / 8;
        let bit = i % 8;
        while index > self.0.len() {
            self.0.push(0);
        }
        self.0[i] |= 1 << bit;
    }

    fn unset(&mut self, i: usize) {
        let index = i / 8;
        let bit = i % 8;
        while index > self.0.len() {
            self.0.push(0);
        }
        self.0[i] &= !(1 << bit);
    }

    fn get(&self, i: usize) -> bool {
        let index = i / 8;
        let bit = i % 8;
        assert!(index <= self.0.len());
        (self.0[i] & (1 << bit)) != 0
    }

    fn resize(&mut self) -> usize {
        let len = self.0.len();
        for i in (len..0).rev() {
            if self.0[i] != 0 {
                let mut k = 0;
                for j in (0..8).rev() {
                    if (self.0[i] & (1 << j)) != 0 {
                        break;
                    }
                    k += 1;
                }
                let len = self.0.len() - 1;
                let rem = len - i;
                for _ in 0..rem {
                    self.0.pop();
                }
                self.0.shrink_to_fit();
                return rem + (7 - k);
            }
        }
        let mut k = 0;
        for j in (0..8).rev() {
            if (self.0[j] & (1 << j)) != 0 {
                break;
            }
            k += 1;
        }
        7 - k
    }
}

pub struct VecStorage<T: Sized> {
    components: Vec<T>,
    unused: BitMap,
}

impl<T: Component> ComponentStorage for VecStorage<T> {
    type Comp = T;

    fn new() -> Self {
        VecStorage {
            components: Vec::new(),
            unused: BitMap::new(),
        }
    }


    fn get(&self, i: usize) -> Option<&T> {
        if self.unused.get(i) {
            Some(&self.components[i])
        } else {
            None
        }
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if self.unused.get(i) {
            Some(&mut self.components[i])
        } else {
            None
        }
    }

    fn insert(&mut self, i: usize, value: T) {
        if self.unused.get(i) {
            panic!("Can not insert a component in place of an existing one.");
        }
        if self.components.capacity() < i {
            self.components.reserve(i + 1);
        }
        unsafe {
            self.components.set_len(i + 1);
        }
        // The value was not used so it was already droped
        // Thus we cant run the drop on the value which is currently there
        mem::forget(mem::replace(&mut self.components[i], value));
        self.unused.set(i);

    }

    fn remove(&mut self, i: usize) {
        self.unused.unset(i);
        // drop the removed value without
        // actually removing anything from the vector
        unsafe {
            mem::replace(&mut self.components[i], mem::uninitialized());
        }
    }

    fn resize(&mut self) {
        let rem = self.unused.resize();
        unsafe {
            let len = self.components.len();
            self.components.set_len(len - rem);
        }
        self.components.shrink_to_fit();
    }
}
