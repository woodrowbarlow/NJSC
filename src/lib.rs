#![crate_type = "dylib"]

extern crate libc;

mod jni;

#[allow(non_snake_case)]
pub mod exports {
    use jni::{JNIEnv, JObject, JInt, JStr, JString};

    #[no_mangle]
    pub extern fn Java_me_wbarlow_njsc_NJSC_info(env: &JNIEnv, _this: JObject) {
        let (major, minor) = env.get_version();
        println!("hello from rust! (JNI version {}.{})", major, minor);
    }

    #[no_mangle]
    pub extern fn Java_me_wbarlow_njsc_NJSC_hasArgument(_env: &JNIEnv, _this: JObject, i: JInt) {
        println!("this prints the number {}.", i);
    }

    #[no_mangle]
    pub extern fn Java_me_wbarlow_njsc_NJSC_sum(_env: &JNIEnv, _this: JObject, a: JInt, b: JInt) -> JInt {
        println!("we're adding {} and {}.", a, b);
        return a + b;
    }

    #[no_mangle]
    pub extern fn Java_me_wbarlow_njsc_NJSC_makeString(env: &JNIEnv, _this: JObject) -> JString {
        return env.new_string("this string was made in rust!").to_jstring();
    }

    #[no_mangle]
    pub extern fn Java_me_wbarlow_njsc_NJSC_concat(env: &JNIEnv, _this: JObject, a: JString, b: JString) -> JString {
        match (JStr::from_jstring(env, a), JStr::from_jstring(env, b)) {
            (Some(ajs), Some(bjs)) => {
                println!("concatenating strings of length {} and {}", ajs.len(), bjs.len());

                let mut rs = ajs.to_string();
                rs.push_str(&*bjs.to_string());
                return env.new_string(&*rs).to_jstring();
            },
            (None, _) => {
                println!("a is null");
            },
            (_, None) => {
                println!("b is null");
            }
        }
        return env.new_string("").to_jstring();
    }
}
