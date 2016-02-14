/** 
 * Root is the data structure in which all data which needs to be saved between frames is saved.
 * The idea is to have a single target for changing.
 * Disconnecting the data from the call structure.
 * this means that almost the whole engine is nothing but function calls to other functions
 * Makeing managing references a lot easier.
 *
 * The idea is to make the whole thing serializable making easy saving, inpections and reflections
 * possible.
 */

trait RootData: Sized{
    fn new() -> Self;
}

struct Root<Game,Res,Setting>{
    //game logic and actors
    game: Game,
    //cached files and loaded resources.
    resources: Res,
    //overal settings for the engine.
    setting: Setting,
}

