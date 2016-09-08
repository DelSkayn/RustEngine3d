//! mod for the container for the references to render objects.
//! responisiblities include:
//!
//! - Keeping track of render objects
//! - copying rendering objects to local version
//!

use super::task::sync::mutate_inspect::{self,Mutator,Inspector}

use super::RenderObject;


pub struct Objects{
    objects: Vec<Inspector<Render>
}
