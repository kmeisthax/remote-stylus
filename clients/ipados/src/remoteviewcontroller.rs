//! Remote Control View Controller

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL, YES};
use objc::{class, msg_send, sel, sel_impl};

extern "C" fn rsrs_remote_view_controller_load_view(this: &Object, _cmd: Sel) {
    let view_class = class!(RSRSStylusCaptureView);
    let view: *mut Object = unsafe { msg_send![view_class, alloc] };

    let _ : () = unsafe { msg_send![view, init] };

    let _ : () = unsafe { msg_send![this, setView:view] };
}

/// Define RSRSRemoteViewController
pub fn def_class() -> &'static Class {
    let superclass = class!(UIViewController);
    let mut decl = ClassDecl::new("RSRSRemoteViewController", superclass).unwrap();

    unsafe {
        decl.add_method(
            sel!(loadView),
            rsrs_remote_view_controller_load_view as extern "C" fn(&Object, Sel),
        );
    }

    decl.register()
}
