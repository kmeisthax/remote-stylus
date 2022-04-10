//! Remote Stylus App Scene Delegate

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Protocol, Sel, BOOL, YES};
use objc::{class, msg_send, sel, sel_impl};

extern "C" fn rsrs_app_scene_delegate_scene_will_enter_foreground(
    _this: &Object,
    _cmd: Sel,
    scene: *mut Object,    //UIScene
) {
    let window: *mut Object = unsafe { msg_send![class!(UIWindow), alloc] };
    let _: () = unsafe { msg_send![window, initWithWindowScene:scene] };
    let _: () = unsafe { msg_send![window, makeKeyAndVisible] };

    let vc = class!(RSRSRemoteViewController);
    let myvc: *mut Object = unsafe { msg_send![vc, alloc] };
    let _: () = unsafe { msg_send![myvc, init] };

    let _: () = unsafe { msg_send![window, setRootViewController: myvc] };
}

/// Define RSRSAppSceneDelegate
pub fn def_class() -> &'static Class {
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("RSRSAppSceneDelegate", superclass).unwrap();

    let window_scene_protocol = Protocol::get("UIWindowSceneDelegate").unwrap();
    decl.add_protocol(window_scene_protocol);

    unsafe {
        decl.add_method(
            sel!(sceneWillEnterForeground:),
            rsrs_app_scene_delegate_scene_will_enter_foreground
                as extern "C" fn(&Object, Sel, *mut Object),
        );
    }

    decl.register()
}
