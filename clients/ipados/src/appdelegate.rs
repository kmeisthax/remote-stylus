//! Remote stylus app delegate

use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL, YES, NO};
use objc::{class, sel, sel_impl, msg_send};
use std::ffi::c_void;

extern "C" fn rsrs_app_delegate_application_did_finish_launching_with_options(
    _this: &Object,
    _cmd: Sel,
    _app: *mut Object,            //UIApplication
    _launch_options: *mut Object, //NSDictionary<UIApplicationLaunchOptionsKey, Id<Object>>,
) -> BOOL {
    let uiapp = class!(UIApplication);

    let uiapp: *mut Object = unsafe { msg_send![uiapp, sharedApplication] };
    let hasscenes: BOOL = unsafe { msg_send![uiapp, supportsMultipleScenes] };
    if hasscenes == NO {
        panic!("Wait this is an iPhone... or an iPod Touch. Do those still exist?");
    }

    YES
}

const DEFAULT_SCENE_CFG_NAME: &str = "Stylus Capture";
const UTF8_ENCODING: usize = 4;

#[link(name = "UIKit", kind = "framework")]
extern "C" {
    static UIWindowSceneSessionRoleApplication: *const Object;
}

extern "C" fn rsrs_app_delegate_application_configuration_for_connecting_scene_session_options(
    _this: &Object,
    _cmd: Sel,
    _app: *mut Object,            //UIApplication
    _scene_session: *mut Object,  //UISceneSession
    _options: *mut Object,        //UISceneConnectionOptions
) -> *mut Object //UISceneConfiguration
{
    let scene_cfg_cls = class!(UISceneConfiguration);
    let string_cls = class!(NSString);

    let bytes = DEFAULT_SCENE_CFG_NAME.as_ptr() as *const c_void;
    let len = DEFAULT_SCENE_CFG_NAME.len();

    let config_name: *mut Object = unsafe { msg_send![string_cls, alloc] };
    let config_name: *mut Object = unsafe { msg_send![config_name, initWithBytes:bytes length:len encoding:UTF8_ENCODING ] };

    let scene_cfg = unsafe { msg_send![scene_cfg_cls, configurationWithName:config_name sessionRole: UIWindowSceneSessionRoleApplication] };

    scene_cfg
}

/// Define RSRSAppDelegate
pub fn def_class() -> &'static Class {
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("RSRSAppDelegate", superclass).unwrap();

    unsafe {
        decl.add_method(
            sel!(application:didFinishLaunchingWithOptions:),
            rsrs_app_delegate_application_did_finish_launching_with_options
                as extern "C" fn(&Object, Sel, *mut Object, *mut Object) -> BOOL,
        );
        decl.add_method(
            sel!(application:configurationForConnectingSceneSession:options:),
            rsrs_app_delegate_application_configuration_for_connecting_scene_session_options
                as extern "C" fn(&Object, Sel, *mut Object, *mut Object, *mut Object) -> *mut Object,
        );
    }

    decl.register()
}
