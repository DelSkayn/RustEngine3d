# Assets

## Identification.
How can we identify an asset from an other asset. Currently the engine identifies assets by the name given to them. However this solutions cannot be the final one.
Because multiple assets can be placed in a single file. For instance a obj file can contain both a mesh and a material. Also what name do we give assets which are refereneced by 
an other asset. Again the obj files can reference textures and material files. Those files cannot be referenced by name. So how do we deel with that?

I currently see the following options
 
 * Append the given name with an increasing number for each referenced file.
 * Create unique number as identifier as assets are created.
