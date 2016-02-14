///
/// The Kernal
///
/// The heart of the game engine.
/// This is where the blood of cpu time is pumped through the engine.
/// This is where the systems live.
///

use super::*;

enum Error{
    NoGameNoLive,
}

type Result<T> = Result<T,Error>;

pub struct KernalFactory<'a,Gam,Win,Ren,Res>
where Gam: Game,
      Win: Tick,
      Ren: System,
      Res: System{

          game: Option<Gam>,
          window: Option<Win>,
          render: Option<Ren>,
          resource: Option<Res>,
      }

impl<'a,Gam,Win,Ren,Res> KernalFactory<'a,Gam,Win,Ren,Res>
where Gam: Game,
      Win: Tick+Happening,
      Ren: System,
      Res: System{

          fn new() -> Self{
              KernalFactory{
                  game:     None,
                  window:   None,
                  render:   None,
                  resource: None,
              }
          }

          fn with_game(self,game: Game) -> Self{
              self.game = Some(game);
              self
          }
          fn with_window(self,window: Win) -> Self{
              self.game = Some(game);
              self
          }

          fn with_render(self,render: Ren) -> Self{
              self.game = Some(game);
              self
          }

          fn with_resource(self,game: Res) -> Self{
              self.game = Some(game);
              self
          }

          fn build(self) -> Result<Kernal>{
              let game = match self.game{
                  Some(x) => x,
                  None => {return Err(Error::NoGameNoLive);},
              };
              let win = match self.window{
                  Some(x) => x,
                  None => NullTick,
              };
              let render = match self.render{
                  Some(x) => x,
                  None => NullSystem,
              }
              let resource = match self.resource{
                  Some(x) => x,
                  None => NullSystem,
              }
              Kernal{
              }
          }

      }

pub struct Kernal<'a,Gam,Win,Ren,Res>
where Gam: Game,
      Win: Tick,
      Ren: System,
      Res: System{

          game: Gam,
          window: Win,
          render: Ren,
          resource: Res,
      }

impl Kernal{
    fn new() -> Self{
    }
}
