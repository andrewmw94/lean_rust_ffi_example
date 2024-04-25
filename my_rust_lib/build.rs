use std::env;
fn main() {
    let lean_dir = env::var("LEAN_LIB_DIR").expect("Error: LEAN_LIB_DIR not defined");
    println!("cargo:rustc-link-search=native=../MyLeanLib/.lake/build/lib");
    println!("cargo:rustc-link-search=native={lean_dir}");
}
