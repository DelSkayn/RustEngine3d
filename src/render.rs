
use super::glium;

use std::rc::Rc;
use std::cell::RefCell;

use super::window::Window;

pub struct RenderEngine{
    window: Rc<Window>,
}
