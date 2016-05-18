extern crate tungsten;

use self::tungsten::Game;

struct TestGame;

impl Game for TestGame{
    fn new() -> Self{
        TestGame
    }
}


fn main(){
    tungsten::Engine::<TestGame>::Go();
}
