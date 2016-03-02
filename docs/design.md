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

Tungsten will use job/task-based multithreading. The idea is to have root be an immutable
data structure task can use to grab data. Systems schedule the jobs and deal with 
syncronisation. The task them-selfs are pieces of work which can be independenently 
executed. 

When a system wants a job to read something from a stream for instance, it allocates data 
on the root and hands the job a reference to it, The job then takes ownership of the data
and does what ever it needs to do. once it is done it returns the ownership to the root
and the system can see the result.
