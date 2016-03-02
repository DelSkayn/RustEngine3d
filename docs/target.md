Target
======

My engine targets a specific platform: PC.

## Min Specs

* SSE3 Compliant Cpus
* Opengl 3.3 Compliant Gpus
* 4gb Mem
* Linux/Windows/Mac
* 4+ cores.

## Resulting Code Choises

### SSE3

SSE3 specifies a set of supported SIMD instructions. 
These instructions are used for vast vector operations. 
SIMD stands for Single Instruct Multiple Data, allowing the whole vector to be able to multiplied at once.

### OGL 3.3

Opengl 3.3 specifies a set of funtions to be able control gpu's. 
Specificly Ogl 3.3 is the version is which the fixed pipeline is depreciated in favor of shaders.

### Linux/Windows/Mac

Luckely rust code is platform independent, and this engine will also be.
However this means that curtain features of rust which aren't platform independent. 
