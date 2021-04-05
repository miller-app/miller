# Miller

Modern front-end for Pure Data audio programming environment.




## Build

Only **macOS**, **Ubuntu** and **Windows** are supported at the moment.

`libsndfile` should be installed in your system.

For Windows or unspecific location of `libsndfile`, you should define
`LIBSNDFILE_PATH`.

Additionally on Windows you need **LLVM** and **MSYS2**. And only
**x86_64-pc-windows-gnu** is supported (i.e. build it with 
`cargo build --target x86_64-pc-windows-gnu`). To make `sndfile.h` discoverable, 
you can add its directory to `CPLUS_INCLUDE_PATH`.

CI configuration ([ci.yml](.github/workflows/ci.yml)) can be used as a reference
for platform-specific details.
