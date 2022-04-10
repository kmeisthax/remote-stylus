//! Remote Stylus Capture View

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL, YES};
use objc::{class, sel, sel_impl, Encoding, Encode};

pub type CGFloat = f64;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

unsafe impl Encode for CGPoint {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("{CGPoint=dd}") }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

unsafe impl Encode for CGSize {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("{CGSize=dd}") }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

unsafe impl Encode for CGRect {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("{CGSize=dddd}") }
    }
}

extern "C" fn rsrs_stylus_capture_view_draw_rect(_this: &Object, _sel: Sel, _rect: CGRect) {

}

/// Define RSRSStylusCaptureView
pub fn def_class() -> &'static Class {
    let superclass = class!(UIView);
    let mut decl = ClassDecl::new("RSRSStylusCaptureView", superclass).unwrap();

    unsafe {
        decl.add_method(
            sel!(drawRect:),
            rsrs_stylus_capture_view_draw_rect as extern "C" fn (&Object, Sel, CGRect)
        );
    }

    decl.register()
}
