//!
//! The Kernal
//!
//! The heart of the game engine.
//! This is where the blood of cpu time is pumped through the engine.
//! This is where the systems live.
//!
//!

struct Kernal<G,W,C,R,P,RES,S>{
    game: G,
    window: W,
    console: C,
    render: R,
    physics: P,
    resource: RES,
    settings: S,
}
