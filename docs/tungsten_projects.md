Tungsten project docs
=====================

Choises to be made:

what file format do we use?
* TOML
    We can use cargo's own library which is thoughly tested. 
    Nice fore settings. less good at objects and other values.
* JSON
    Nicely readable, Well known, also good at discribing objects. 
    Supported in serde.
    Can be somewhat of a clusterfuck.
* XML
    Often used. Old and well known. Uses a lot of extra data 
    compered to equivilent JSON. Harder to read. 
    Hard to parse correctly.
