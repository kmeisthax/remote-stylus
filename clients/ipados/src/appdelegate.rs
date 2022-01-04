//! Remote stylus app delegate

use objc::declare::ClassDecl;
use objc::runtime::{BOOL, YES};
use objc::Class;
use objc_foundation::{NSDictionary, NSString};
use objc_id::Id;

type UIApplicationLaunchOptionsKey = NSString;

fn application_did_finish_launching_with_options(
    app: *mut UIApplication,
    launch_options: *mut NSDictionary<UIApplicationLaunchOptionsKey, Id>,
) -> BOOL {
    YES
}

/// Define RSRSAppDelegate
fn def_class() -> Class {
    let superclass = class!(UIApplicationDelegate);
    let mut decl = ClassDecl::new("RSRSAppDelegate", superclass).unwrap();

    unsafe {
        decl.add_method(
            sel!(application:didFinishLaunchingWithOptions:),
            application_did_finish_launching_with_options,
        );
    }

    decl.register();

    decl
}
