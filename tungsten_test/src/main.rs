extern crate tungsten;

use self::tungsten::Game;

struct TestGame;

impl Game for TestGame{}


fn main(){
    tungsten::Engine::go(TestGame);
}
