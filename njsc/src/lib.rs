#![crate_type = "dylib"]

use std::any::Any;

#[no_mangle]
pub extern fn Java_tests_NJSC_info(jre: *const Any, class: *const Any) {
    println!("hello from rust!");
}

#[no_mangle]
pub extern fn Java_tests_NJSC_hasArgument(jre: *const Any, class: *const Any, i: i32) {
	println!("this prints the number {}.", i);
}

#[no_mangle]
pub extern fn Java_tests_NJSC_sum(jre: *const Any, class: *const Any, a: i32, b: i32) -> i32 {
	println!("we're adding {} and {}.", a, b);
	return (a + b);
}
