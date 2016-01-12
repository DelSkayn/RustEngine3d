extern crate engine3d;

use self::engine3d::engine;

mod test_game;

fn main(){
    let mut engine = engine::Engine::new::<test_game::TestGame>();
    engine.run();
}
