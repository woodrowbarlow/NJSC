mod sys;

use std::ffi;

pub type JObject = sys::jobject;
pub type JInt = sys::jint;
pub type JString = sys::jstring;

#[repr(C)]
pub struct JNIEnv {
    env: sys::JNIEnv,
}

impl JNIEnv {
    pub fn get_version(&self) -> (u16, u16) {
        let version = unsafe { ((*self.env).GetVersion)(&self.env) as u32 };
        // major version in upper 16 bits
        let major = ((version >> 16) & 0xffff) as u16;
        // minor version in lower 16 bits
        let minor = (version & 0xffff) as u16;

        return (major, minor);
    }

    pub fn new_string(&self, s: &str) -> JStr {
        // CString ensures presence of NUL byte
        let cs = ffi::CString::new(s).unwrap();
        return JStr {
            js: unsafe { ((*self.env).NewStringUTF)(&self.env, cs.as_ptr()) },
            env: &self.env,
        }
    }
}

pub struct JStr<'a> {
    js: sys::jstring,
    env: &'a sys::JNIEnv,
}

impl<'a> JStr<'a> {
    pub fn from_jstring<'b>(env: &'b JNIEnv, s: JString) -> JStr<'b> {
        JStr { js: s, env: &env.env }
    }

    /// Gets the length of the string in bytes
    pub fn len(&self) -> usize {
        unsafe {
            return ((**self.env).GetStringUTFLength)(self.env, self.js) as usize;
        }
    }

    pub fn to_jstring(&self) -> JString {
        return self.js;
    }

    // TODO: provide a function which returns a copy-on-write string pointer
}

impl<'a> ToString for JStr<'a> {
    fn to_string(&self) -> String {
        unsafe {
            let mut is_copy = sys::JNI_FALSE;
            let slice = ffi::CStr::from_ptr(
                ((**self.env).GetStringUTFChars)(self.env, self.js, &mut is_copy));
            let s = String::from_utf8_lossy(slice.to_bytes()).into_owned();
            ((**self.env).ReleaseStringUTFChars)(self.env, self.js, slice.as_ptr());

            return s;
        }
    }
}
