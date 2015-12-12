use super::glium::backend::glutin_backend::{
    GlutinFacade,
};
use super::glium::DisplayBuild;

use super::glium::glutin::WindowBuilder;

use glium::glutin::Event;
use glium::glutin::ElementState;
use glium::glutin::MouseButton;
use glium::glutin::MouseScrollDelta;
use glium::glutin::VirtualKeyCode;

use super::input::Button;
use super::input::Key;
use super::input::Mouse;
use super::input::KeyBoard;

pub struct Window{
    window: GlutinFacade,
}

impl Window{
    pub fn new() -> Self{
        let builder = WindowBuilder::new().with_dimensions(800,600);
        Window{
            window: builder.build_glium().unwrap(),
        }
    }

    pub fn get_display<'a>(&'a self) -> &'a GlutinFacade{
        &self.window
    }

    fn match_button(ev: MouseButton) -> Button{
    use super::input::Button::*;
        match ev{
            MouseButton::Left => Left,
            MouseButton::Right => Right,
            MouseButton::Middle => Middle,
            MouseButton::Other(0) => Button4,
            MouseButton::Other(1) => Button5,
            MouseButton::Other(2) => Button6,
            MouseButton::Other(_) => Unkown,
        }
    }

    fn match_key(vkc: VirtualKeyCode) -> Key{
    use super::input::Key::*;
        match vkc{
           VirtualKeyCode::Key0     => Key0, VirtualKeyCode::Key1     => Key1, VirtualKeyCode::Key2     => Key2,
           VirtualKeyCode::Key3     => Key3, VirtualKeyCode::Key4     => Key4, VirtualKeyCode::Key5     => Key5,
           VirtualKeyCode::Key6     => Key6, VirtualKeyCode::Key7     => Key7, VirtualKeyCode::Key8     => Key8,
           VirtualKeyCode::Key9     => Key9, VirtualKeyCode::A        => A, VirtualKeyCode::B        => B,
           VirtualKeyCode::C        => C, VirtualKeyCode::D        => D, VirtualKeyCode::E        => E,
           VirtualKeyCode::F        => F, VirtualKeyCode::G        => G, VirtualKeyCode::H        => H,
           VirtualKeyCode::I        => I, VirtualKeyCode::J        => J, VirtualKeyCode::K        => K,
           VirtualKeyCode::L        => L, VirtualKeyCode::M        => M, VirtualKeyCode::N        => N,
           VirtualKeyCode::O        => O, VirtualKeyCode::P        => P, VirtualKeyCode::Q        => Q,
           VirtualKeyCode::R        => R, VirtualKeyCode::S        => S, VirtualKeyCode::T        => T,
           VirtualKeyCode::U        => U, VirtualKeyCode::V        => V, VirtualKeyCode::W        => W,
           VirtualKeyCode::X        => X, VirtualKeyCode::Y        => Y, VirtualKeyCode::Z        => Z,
           VirtualKeyCode::F1       => F1, VirtualKeyCode::F2       => F2, VirtualKeyCode::F3       => F3,
           VirtualKeyCode::F4       => F4, VirtualKeyCode::F5       => F5, VirtualKeyCode::F6       => F6, 
           VirtualKeyCode::F7       => F7, VirtualKeyCode::F8       => F8, VirtualKeyCode::F9       => F9,
           VirtualKeyCode::F10      => F10, VirtualKeyCode::F12      => F12, VirtualKeyCode::Space    => Space,
           VirtualKeyCode::Up       => Up, VirtualKeyCode::Right    => Right, VirtualKeyCode::Down     => Down,
           VirtualKeyCode::Left     => Left, VirtualKeyCode::LControl => LCtrl, VirtualKeyCode::LShift   => LShift,
           VirtualKeyCode::LAlt     => LAlt, VirtualKeyCode::RControl => RCtrl, VirtualKeyCode::RShift   => RShift,
           VirtualKeyCode::RAlt     => RAlt, VirtualKeyCode::Escape   => Esc, _ => UnkownKey,
        }
    }
}

/*
    fn get_events(&self) -> Vec<BaseEvent>{
        self.window.poll_events().filter_map(
            |ev|match ev{
                Event::Resized(w,h) => Some(BaseEvent::Resize(w,h)),
                Event::Closed => Some(BaseEvent::Quit),
                Event::MouseMoved((w,h)) => 
                    Some(BaseEvent::Mouse(Mouse::Move([w as f32,h as f32]))),
                Event::MouseInput(ElementState::Pressed, x) => 
                    Some(BaseEvent::Mouse(Mouse::Pressed(Window::match_button(x)))),
                Event::MouseInput(ElementState::Released, x) => 
                    Some(BaseEvent::Mouse(Mouse::Released(Window::match_button(x)))),
                Event::MouseWheel(MouseScrollDelta::LineDelta(_,y)) =>  
                    Some(BaseEvent::Mouse(Mouse::Wheel(y))),
                Event::MouseWheel(MouseScrollDelta::PixelDelta(_,y)) =>  
                    Some(BaseEvent::Mouse(Mouse::Wheel(y))),
                Event::KeyboardInput(ElementState::Pressed,_,Some(x)) =>
                    Some(BaseEvent::KeyBoard(KeyBoard::Pressed(Window::match_key(x)))),
                Event::KeyboardInput(ElementState::Released,_,Some(x)) =>
                    Some(BaseEvent::KeyBoard(KeyBoard::Released(Window::match_key(x)))),
                Event::ReceivedCharacter(x) =>
                    Some(BaseEvent::KeyBoard(KeyBoard::Character(x))),
                _ => None,
            }).collect()
    }
*/
