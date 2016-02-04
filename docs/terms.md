# Glossary

In programming there are a lot of terms and sometimes what those terms mean different things to different people. This file is used to keep track of the different terms used in the engine documentation. 

## Code related

### Resource

All data handled in the engine during runtime which is not player input. Often loaded from a file on runtime. The current engine has _ different resources.

* Render
* Sound
* Text
* Settings

### System

A certain part of the engine handeling one of the core functionalities of the engine. The current engine has _ parts:

* Core
    Actually not a system but menages all the other systems.
* Rendering.
    Handles rendering the game as wel as menaging rendering resources on the gpu.
