Parallel Logic
==============

Tungsten strifes to be as parralel as posible. 
Scaling to a lot of threads is a large priority.
In order to acomplish this tungstens logic is paralel.

The logic component of tungsten is an entity component system.
The entity component system consists of 3 primary parts
 * Components: Data
 * Systems: Functions
 * Entitys: Id's tying Components together.

The logic systems of tungsten tries to execute Systems as much paralel as possible.
This means that there are no gaurenties as to the order of execution of systems.
There are however a few rules. 
Systems dependent on others will be executed after the 
