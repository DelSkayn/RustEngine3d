extern crate tungsten;

use tungsten::Registry;
use self::tungsten::Game;

struct TestGame;

impl Game for TestGame{
    fn new() -> Self{
        Registry::set("very.deeply.nested.variable","hallo".to_string());
        let s: String = Registry::get("very.deeply.nested.variable").unwrap();
        println!("test: {}",s);
        TestGame
    }
}


fn main(){
    tungsten::Engine::<TestGame>::Go();
}
