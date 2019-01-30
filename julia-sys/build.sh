#!/bin/bash
set -eux

bindgen \
  --whitelist-function "jl_.*" \
  --whitelist-type "jl_.*"     \
  --whitelist-var "jl_.*"      \
  /usr/include/julia/julia.h   \
  > src/julia.rs
