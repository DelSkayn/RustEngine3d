# TODO

#Engine:
* Core:
..* Events:
    ...Redesign the way events are feeded to the game loop.
    ...Multiple sources multiple targets? If so how what do we pass, All? filtered? only requested? If so how do we filter events efficiently? if not how does passing all events multiple time effect performence.
..* Game Loop:
    ...The Game loop is currently a mess. Meshes loaded, matrixs set, these don't belong there. We need to offload these things to a game object and clean that all up.
..* Processes:
    ...Implement an form of processes handeling. Might also be usefull for handeling multithreading.
* Debug:
* Profile:
..* Memory:
    ...Memory debugging utilities would be very usefull. However with the current way rust is handeling allocation it will be a bit difficult. Rust doesnt allow custom allocators, but there are things like stack allocaters so it may be a possible to do some memory stuff.  
..* CPU:
    ...A profiler is a need is a game engine.
* Resources:
..* Files:
    ...We need an abstraction over files so that resources can be packed to gather in a zip or just put in individual files for version control.
..* Formats:
    ...Unfortunatly collada, obj and other mesh format are not that easy to load. So it will be nessary to roll an custom format for this engine.
..* Managing:
    ...Resources cant be loaded every time you need them, so we need a way to manage there lifetime and load in a way that allows the game engine to continue running.
