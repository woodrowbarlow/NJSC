#![crate_type = "dylib"]
use std::any::Any;

#[no_mangle]
pub extern fn Java_tests_NJSC_test(env: *const Any, jclass: *const Any) {
	println!("hello from rust");
}
