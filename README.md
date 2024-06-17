## Instructions

```shell
cd MyLeanLib
lake build MyLeanLib:static
cd ../my_rust_lib
source set_env_vars.sh
cargo run
```


## Caveats
Multiple threads won't work prior to Lean 4.8.0. This affects `cargo test`.
To use Lean 4.8.0, you can use https://github.com/andrewmw94/lean-sys/tree/main until this gets merged upstream.
