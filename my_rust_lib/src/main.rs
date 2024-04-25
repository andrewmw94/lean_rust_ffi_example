use lean_sys::{
    lean_dec, lean_dec_ref, lean_initialize_runtime_module_locked,
    lean_io_mark_end_initialization, lean_io_mk_world, lean_io_result_is_ok,
    lean_io_result_show_error, lean_mk_string, lean_object, lean_string_cstr,
};
use std::ffi::{CStr, CString};
use std::{sync::Once, thread};

#[link(name = "MyLeanLib")]
extern "C" {
    // fn myFunNoArg() -> *mut lean_object;
    fn myFunOneArg(s: *mut lean_object) -> *mut lean_object;
    fn initialize_MyLeanLib(i: u8, s: *mut lean_object) -> *mut lean_object;
}

static START: Once = Once::new();

fn init(i: u8) {
    println!("Init {i}");

    START.call_once(|| {
        unsafe {
            println!("Once {i}");
            lean_initialize_runtime_module_locked();
            let builtin: u8 = 1;
            let res = initialize_MyLeanLib(builtin, lean_io_mk_world());
            if lean_io_result_is_ok(res) {
                lean_dec_ref(res);
            } else {
                lean_io_result_show_error(res);
                lean_dec(res);
                panic!("Failed to initialize Lean");
            }
            lean_io_mark_end_initialization();
        };
    });
}

fn lean_obj_p_to_rust_string(lean_str_obj: *mut lean_object) -> String {
    let lean_obj_p = unsafe { lean_string_cstr(lean_str_obj) };
    let lean_obj_cstr = unsafe { CStr::from_ptr(lean_obj_p as *const i8) };
    let rust_string = lean_obj_cstr
        .to_str()
        .expect("failed to convert Lean object to string")
        .to_owned();
    unsafe {
        lean_dec(lean_str_obj);
    };
    rust_string
}

fn foo(i: u8) {
    println!("Hello {i}");

    init(i);
    let cstring = CString::new("Hello from Rust").expect("CString::new failed");
    let input_to_lean = unsafe{lean_mk_string(cstring.as_ptr() as *const u8)};
    let response_lean_obj_p = unsafe {myFunOneArg(input_to_lean)};
    // Lean decrements the reference count for `input_to_lean`
    let response_rust_string = lean_obj_p_to_rust_string(response_lean_obj_p);
    // We decremented the reference count for `response_lean_obj_p` in the above function
    
    println!("Response: {response_rust_string}");
    println!("Good bye {i}");
}

fn main() {
    let h1 = thread::spawn(|| {
        foo(1);
    });

    // Multiple threads won't work until https://github.com/leanprover/lean4/pull/3632 is released

    // let h2 = thread::spawn(|| {
    //     foo(2);
    // });

    h1.join().unwrap();
    // h2.join().unwrap();
}
