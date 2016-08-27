Design
======

This is document is a place where i place my design ideas as i am designing the engine.
It will document overal engine layout and design principals.

# Keywords

* Multithreaded

...Cpus have more and more cores it is only logical to make use of those cores.
...In order to be able to use multi-core cpus to there fullest the engine will be designed 
...with multi-threading in mind.

* Data driven.

...You can't go near any performance related code without hearing about Data driven design.
...The engine will try to use data driven desigen were possible. Of course like everthing
...in programming data oriented design is a tradeoff. Data oriented design is most of the
...time very verbose. Rusts macro's might help with that but it is still not the quickest 
...to write code.

## Multithreading

Tungste will heavely make use of the task-rs framework for multithread. Allowing jobs to 
be executed in parallel while incurring as small a cost as possible.
