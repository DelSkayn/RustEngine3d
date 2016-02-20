use super::engine3d::Game;

pub struct TestGame{
    temp: u8,
}

impl Game for TestGame{
    fn new() -> Self{
        TestGame{
            temp: 3,
        }
    }
    fn render(&mut self){

    }
    fn update(&mut self){

    }
}
