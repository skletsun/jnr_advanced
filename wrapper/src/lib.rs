extern crate jni;
extern crate second;

use jni::{
    objects::JClass,
    JNIEnv
};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_jni_proto_JniWrapper_div(_env: JNIEnv, _class: JClass, a: i32, b: i32) -> i32 {
    second::div_native(a, b)
}
