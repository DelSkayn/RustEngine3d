Problems
========

this document is meant to be a place i document the sollutions i come up with when dealing with the problems presented in the game engine. The document is mostly for my own sanity so i don't repeat the same mistakes i made and repaired earlier. 

## 1 Resources everwhere.
### The problem
The problem originates from the resource system. The engine needs to be able to load and manage resources. In order to accomplish this it needs a place systems can retrieve resource data. This place needs to be globaly accesable to all systems who need resource data. Examples: Renderer, logic, sound, etc. 
### solutions
* Use AtomicOption
    AtomicOption can be used to have make sure the 
* Split root in an sync part and a async part.
    Refering to resources doesnt nessacerly need to be done async. When can have systems handle retriefing the resource and the giving them to async as nessesary. Requires 3

## 2 Job Syncronistation
### The problem 
The Kernal needs a way to ensure that certain jobs only start when other jobs are finished.

## 3 After jobs
### The problem
The current kernal has no way to run systems after jobs are done. 
