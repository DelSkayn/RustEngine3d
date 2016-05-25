
#[derive(Copy,Clone,PartialEq,Debug)]
pub enum InputEvent{
    Mouse(Mouse),
    KeyBoard(KeyBoard),
    GamePad,
    Vr,//i wish
    Other,
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum KeyBoard{
    Pressed(Key),
    Released(Key),
    Character(char),
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Key{
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S,
    T, U, V, W, X, Y, Z, LShift, RShift, LCtrl, RCtrl, LAlt,
    RAlt, OpenBracket, CloseBracket, Comma, Tab, Space, F1, F2,
    F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, Up, Down, Left,
    Right, Esc, UnkownKey,
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Mouse{
    Move([f32;2]),
    Wheel(f32),
    Pressed(Button),
    Released(Button),
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Button{
    Left,
    Right,
    Middle,
    Button4,
    Button5,
    Button6,
    Esc,
    Unkown,
}

enum MouseGrabError{

}

struct KeyData{
    pressed: bool,
}

enum MouseState{
    Grabbed,
    Free,
}

struct MouseData{
    state: MouseState,
    pos: [f64;2],
    delta: [f64;2],
}

struct Input{
    raw_que: Vec<InputEvent>,
    key_data: Hashmap<Key,
    mouse_data:
}

impl Input{
    pub fn is_key_pressed(&self,key: Key) -> bool{
        unimplemented!();
    }

    pub fn is_key_released(&self,key: Key) -> bool{
        unimplemented!();
    }

    //grabs the mouse context
    pub fn grab_mouse(&self){
        unimplemented!();
    }

    pub fn is_mouse_grabbed(&self) -> bool{
        unimplemented!();
    }

    //grabs the mouse context
    pub fn release_mouse(&self){
        unimplemented!();
    }

    /// returns the position of the mouse 
    /// relative to the start position when grabbed.
    /// the position only changes while the mouse is grabed
    /// usefull for first person cams
    pub fn mouse_rel_pos(&self) -> [f64; 2]{
        unimplemented!();
    }

    /// returns the position of the mouse 
    /// relative to the position last frame when grabbed.
    /// when the mouse is not grabbed it will always return 0;
    pub fn mouse_rel_delta(&self) -> [f64; 2]{
        unimplemented!();
    }

    /// returns the position relative to the
    /// top left (?) of the window.
    /// Only changes when the mouse is not grabbed.
    /// Usefull for ui.
    pub fn mouse_window_pos(&self) -> [i64; 2]{
        unimplemented!();
    }
}
