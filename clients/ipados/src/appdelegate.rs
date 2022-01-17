//! Remote stylus app delegate

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL, YES};
use objc::{class, sel, sel_impl};

extern "C" fn rsrs_app_delegate_application_did_finish_launching_with_options(
    _this: &Object,
    _cmd: Sel,
    _app: *mut Object,            //UIApplication
    _launch_options: *mut Object, //NSDictionary<UIApplicationLaunchOptionsKey, Id<Object>>,
) -> BOOL {
    YES
}

/// Define RSRSAppDelegate
pub fn def_class() -> &'static Class {
    let superclass = class!(UIApplicationDelegate);
    let mut decl = ClassDecl::new("RSRSAppDelegate", superclass).unwrap();

    unsafe {
        decl.add_method(
            sel!(application:didFinishLaunchingWithOptions:),
            rsrs_app_delegate_application_did_finish_launching_with_options
                as extern "C" fn(&Object, Sel, *mut Object, *mut Object) -> BOOL,
        );
    }

    decl.register()
}
