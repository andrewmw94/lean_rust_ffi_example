## Instructions

```shell
cd MyLeanLib
lake build MyLeanLib:static
cd ../my_rust_lib
source set_env_vars.sh
cargo run
```


## Caveats
Multiple threads won't work until https://github.com/leanprover/lean4/pull/3632 is released. This affects `cargo test`.
