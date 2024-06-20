## Instructions

```shell
cd MyLeanLib
lake build MyLeanLib:static
cd ../my_rust_lib
source set_env_vars.sh
cargo run
```

## Caveats
You need `lean --print-libdir` to point to Lean 4.8.0 to run multiple threads. Just setting `MyLeanLib/lean-toolchain` to Lean 4.8.0 is not sufficient.
