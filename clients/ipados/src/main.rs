use objc::rc::autoreleasepool;
use objc::runtime::{Class, Object};
use std::env::args;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;

mod appdelegate;
mod appscenedelegate;
mod remoteviewcontroller;
mod stylusview;

#[link(name = "UIKit", kind = "framework")]
extern "C" {
    fn UIApplicationMain(
        argc: c_int,
        argv: *const *const c_char,
        principalClassName: *const Object,
        delegateClassName: *const Object,
    ) -> c_int;
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    fn NSStringFromClass(class: *const Class) -> *const Object;
}

fn main() {
    let args = args()
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();
    let c_args = args
        .iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();

    autoreleasepool(|| {
        let rsrs_app_delegate = appdelegate::def_class();
        stylusview::def_class();
        remoteviewcontroller::def_class();
        appscenedelegate::def_class();

        unsafe {
            UIApplicationMain(
                c_args.len() as c_int,
                c_args.as_ptr(),
                ptr::null(),
                NSStringFromClass(rsrs_app_delegate),
            );
        }
    });
}
