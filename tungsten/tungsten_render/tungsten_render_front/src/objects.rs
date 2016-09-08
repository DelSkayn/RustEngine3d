//! mod for the container for the references to render objects.
//! responisiblities include:
//!
//! - Keeping track of render objects
//! - copying rendering objects to local version
//!

use super::task::sync::mutate_inspect::{self, Mutator, Inspector};

use super::RenderObject;
use super::{Generator, GenId};

/// A struct handeling the copying of data for static render objects.
pub struct StaticObjects {
    remote: Vec<Inspector<RenderObject>>,
    local: Vec<(GenId, RenderObject)>,
    ids: Generator,
}

impl StaticObjects {
    /// Create a new object
    pub fn new() -> Self {
        StaticObjects {
            remote: Vec::new(),
            local: Vec::new(),
            ids: Generator::new(),
        }
    }

    /// Add a new object to be used copied when compiling render data.
    pub fn add(&mut self, object: Inspector<RenderObject>) {
        self.remote.push(object);
    }

    /// Proces all registered static render objects.
    pub fn process(&mut self) {
        self.fetch();
        self.cull();
    }

    /// Remove objects which have been dropped by remote.
    ///
    /// EX-STATE: Can only be called when the length of
    ///           remote and local ques are the same.
    fn cull(&mut self) {
        let mut i = 0;
        while i < self.local.len() {
            if self.remote[i].mutator_present() {
                i += 1;
            } else {
                // mutator is dropped remove from list.
                // since we swap remove we need to check the last again.
                self.ids.free(self.local.swap_remove(i).0);
                self.remote.swap_remove(i);
            }
        }
    }

    /// Copy data from remote objects to local buffer
    fn fetch(&mut self) {
        let len = self.local.len();
        // fetch remote objects which have not been addede jet.
        for i in self.remote.len()..len {
            self.local.push((self.ids.next(), self.remote[i].get()));
        }
        // Might want to move this across multiple task.
        // Copy changed.
        for i in 0..len {
            if self.remote[i].changed() {
                self.local[i].1 = self.remote[i].get();
            }
        }
    }
}
