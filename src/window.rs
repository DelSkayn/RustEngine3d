use super::glium::backend::glutin_backend::{
    GlutinFacade,
};
use super::glium::DisplayBuild;

use super::glium::glutin::WindowBuilder;

use glium::glutin::MouseButton;
use glium::glutin::Event as GlutinEvent;
use glium::glutin::MouseScrollDelta;
use glium::glutin::ElementState;
use glium::glutin::VirtualKeyCode;

use super::input::*;

use super::kernal::System;
use super::kernal::EventHandle;

use super::Event;
use super::CoreEvent;

//TODO: Temp testing remove
use super::render::RenderEvent;
use super::time;

use super::profile::ProfileSample;

pub struct Window{
    window: GlutinFacade,
    event: EventHandle,
} 

impl Window{
    pub fn new(event: EventHandle) -> Self{
        let builder = WindowBuilder::new().with_dimensions(800,600).with_vsync();
        Window{
            window: builder.build_glium().unwrap(),
            event: event,
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

impl System for Window{
    fn run(&mut self){
        ProfileSample::new("Window system run");
        for ev in self.window.poll_events(){
            self.event.push(match ev{
                GlutinEvent::Resized(w,h) => Event::Core(CoreEvent::Resize(w,h)),
                GlutinEvent::Closed => Event::Core(CoreEvent::Quit),
                GlutinEvent::MouseMoved((w,h)) => 
                    Event::Input(InputEvent::Mouse(Mouse::Move([w as f32,h as f32]))),
                GlutinEvent::MouseInput(ElementState::Pressed, x) => 
                    Event::Input(InputEvent::Mouse(Mouse::Pressed(Window::match_button(x)))),
                GlutinEvent::MouseInput(ElementState::Released, x) => 
                    Event::Input(InputEvent::Mouse(Mouse::Released(Window::match_button(x)))),
                GlutinEvent::MouseWheel(MouseScrollDelta::LineDelta(_,y)) =>  
                    Event::Input(InputEvent::Mouse(Mouse::Wheel(y))),
                GlutinEvent::MouseWheel(MouseScrollDelta::PixelDelta(_,y)) =>  
                    Event::Input(InputEvent::Mouse(Mouse::Wheel(y))),
                GlutinEvent::KeyboardInput(ElementState::Pressed,_,Some(x)) =>
                    Event::Input(InputEvent::KeyBoard(KeyBoard::Pressed(Window::match_key(x)))),
                GlutinEvent::KeyboardInput(ElementState::Released,_,Some(x)) =>
                    Event::Input(InputEvent::KeyBoard(KeyBoard::Released(Window::match_key(x)))),
                GlutinEvent::ReceivedCharacter(x) =>
                    Event::Input(InputEvent::KeyBoard(KeyBoard::Character(x))),
                _ => continue,
            });
        }
        //TODO: Temp testing remove
        self.event.push(Event::Render(RenderEvent::Frame));

        for e in self.event.into_iter(){
            match e {
                Event::Profile(time) =>{
                    debug!("Profile event window: {}",(time::precise_time_s() - time));
                },
                _ => {},
            }
        }
    }
}
