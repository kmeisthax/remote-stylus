//! Remote Stylus View

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL, YES};
use objc::{class, sel, sel_impl};

/// Define RSRSStylusCaptureView
pub fn def_class() -> &'static Class {
    let superclass = class!(UIView);
    let mut decl = ClassDecl::new("RSRSStylusCaptureView", superclass).unwrap();

    decl.register()
}
