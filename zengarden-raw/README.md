# zengarden-raw

Raw bindings for [ZenGarden fork](https://github.com/miller-app/ZenGarden)
generated by bindgen.




## Build

Depends on **libsndfile**. Also depends on **mingw** on Windows.

`wrapper.hpp` is generated automatically by `./fetch_sources.sh` script. The
script clones zengarden sources, removes unused stuff and generates the
`wrapper.hpp`.