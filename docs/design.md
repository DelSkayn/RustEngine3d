# Design
In this file I document the different design decisions I make while coding this engine.

## General Design Idea's.
 * Data-Oriented design.
 * Multithreaded.
 * EventDriven.

## Explaination

### Data-Oriented Design.

So I resently got a new head-canon, All data which has multiple readers/mutators should not be owned by any of them.
To elaborate: Take position data for a game entity. This data needs to be read by the game logic, Physics engine and the Rendering engine. 
So where does it belong? In its own data structure. The only owner of aliasable data is the root, A node availble to all system. 
What does this solve? Mostly data execs problems, You want to structure systems in a way that makes sence form a calling perspective. However this often means that it is not wel organized for refering to the actual data. So you decouple the data and the system working on it. This means that the actual systems should barely have any data. 
Exceptions: Data which is only refered to inside the same class.  

### Multithreaded.

Modern cpu's have multiple cores. And as time goes on the amount of cores will only increase. So a game engine which wants to be usefull will need to be able to spread out its work across multiple threads. So how does this engine acheive this

### EventDriven.

FoxLight tries to keep its core systems as decoupled as possible.
