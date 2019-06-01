# julia.rs
[![Build Status](https://dev.azure.com/termoshtt/julia.rs/_apis/build/status/termoshtt.julia.rs?branchName=master)](https://dev.azure.com/termoshtt/julia.rs/_build/latest?definitionId=4&branchName=master)

WIP: Rust binding to Julia

Run example
-----------
You have to expose the shared library (*cdylib* in Rust) to julia.
`cargo build` will put the shared library at `target/debug` directory, and you need to specify it:

```
cargo build
LD_LIBRARY_PATH=$PWD/target/debug julia example.jl
```

For `cargo build --release`, it becomes `target/release`.

Link to libjulia.so
--------------------
If you download bare julia binary (e.g. on Ubuntu), you may need to set `LD_LIBRARY_PATH` to link them.
This will be skipped when you use package manager e.g. Yum/DNF, Pacman and others.

```
export LD_LIBRARY_PATH=$HOME/Download/julia-1.1.1/lib:$HOME/Download/julia-1.1.1/lib/julia
```
