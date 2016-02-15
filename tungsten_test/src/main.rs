extern crate tungsten;

use self::tungsten::Game;

struct TestGame;

impl<'a> Game<'a> for TestGame<'a>{

    fn new(_data: &'a mut GameData) -> Self<'a>{
        TestGame
    }

    fn update(){
    };

    fn render(){
    };
    fn init(){
    };
}

fn main(){
    tungsten::TungEngine::<TestGame>::Go();
}
