use crossbeam_channel::{unbounded, Receiver};
use libremotestylus::{
    target, Error, MouseButtons, Point, PointerData, PointerEvent, StreamAction,
};
use std::mem::size_of;
use std::thread;
use windows::Win32::Foundation::{HANDLE, POINT};
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Controls::{
    CreateSyntheticPointerDevice, POINTER_FEEDBACK_DEFAULT, POINTER_TYPE_INFO,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_MOVE,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_VIRTUALDESK, MOUSEEVENTF_XDOWN,
    MOUSEEVENTF_XUP, MOUSEINPUT,
};
use windows::Win32::UI::Input::Pointer::{
    InjectSyntheticPointerInput, POINTER_FLAG_CANCELED, POINTER_FLAG_NEW, POINTER_FLAG_UP,
    POINTER_FLAG_UPDATE, POINTER_INFO,
};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, PostThreadMessageW, TranslateMessage, MAX_TOUCH_COUNT, MSG,
    PT_PEN, PT_TOUCH, WM_APP, XBUTTON1, XBUTTON2,
};

/// List of messages that the target will send to the UI thread.
enum TargetEvent {
    Pointer(PointerEvent),
    Error(Error),
}

struct TargetServicer {
    /// Reciever for events from the target-process thread.
    recv: Receiver<TargetEvent>,

    /// Handle to synthetic touch injection device.
    synthetic_touch: isize,

    /// Handle to synthetic pen injection device.
    synthetic_pen: isize,

    /// The last reported mouse position.
    last_mouse_position: Point,

    /// The last reported mouse button state.
    last_buttons: MouseButtons,
}

impl TargetServicer {
    pub fn setup() -> Self {
        let mainthread_id = unsafe { GetCurrentThreadId() };
        let (send, recv) = unbounded();

        // Start up the event target process.
        // We use the WM_APP message to wake up the main thread and tell it to pump
        // out all the pending target events.
        thread::spawn(move || {
            target(
                |pevent| {
                    send.send(TargetEvent::Pointer(pevent)).unwrap();
                    unsafe { PostThreadMessageW(mainthread_id, WM_APP, 0, 0) };
                },
                |error| {
                    send.send(TargetEvent::Error(error)).unwrap();
                    unsafe { PostThreadMessageW(mainthread_id, WM_APP, 0, 0) };
                },
            );
        });

        // Spin up our pointer injector.
        let synthetic_touch = unsafe {
            CreateSyntheticPointerDevice(PT_TOUCH, MAX_TOUCH_COUNT, POINTER_FEEDBACK_DEFAULT)
        };
        if synthetic_touch == 0 {
            panic!("Could not create synthetic pointer device (touch)");
        }

        let synthetic_pen =
            unsafe { CreateSyntheticPointerDevice(PT_PEN, 1, POINTER_FEEDBACK_DEFAULT) };
        if synthetic_pen == 0 {
            panic!("Could not create synthetic pointer device (pen)");
        }

        Self {
            recv,
            synthetic_touch,
            synthetic_pen,
            last_mouse_position: Point::default(),
            last_buttons: MouseButtons::default(),
        }
    }

    fn update_pointer(&mut self, p: PointerEvent) {
        let mut ptrinfo: POINTER_INFO = Default::default();

        ptrinfo.pointerId = p.stream as u32;
        //TODO: ptrinfo.frameId ..?
        ptrinfo.pointerFlags = match p.stream_action {
            StreamAction::Start => POINTER_FLAG_NEW,
            StreamAction::Update => POINTER_FLAG_UPDATE,
            StreamAction::End => POINTER_FLAG_UP,
            StreamAction::Cancel => POINTER_FLAG_CANCELED,
        };
        //TODO: ptrinfo.hwndTarget?
        ptrinfo.ptPixelLocation = POINT {
            //TODO: DPI un-awareness
            x: p.location.0 as i32,
            y: p.location.1 as i32,
        };
        ptrinfo.ptHimetricLocation = POINT {
            x: p.location.0 as i32,
            y: p.location.1 as i32,
        };
        ptrinfo.ptPixelLocationRaw = ptrinfo.ptPixelLocation;
        ptrinfo.ptHimetricLocationRaw = ptrinfo.ptHimetricLocation;
        ptrinfo.dwTime = 0;
        ptrinfo.historyCount = 0; //TODO: Maybe ask Laminar how many pointer events we skipped?
        ptrinfo.dwKeyStates = 0;
        ptrinfo.PerformanceCount = 0;
        ptrinfo.ButtonChangeType = 0; //TODO: ¯\_(ツ)_/¯

        match p.data {
            #[allow(clippy::field_reassign_with_default)]
            PointerData::Finger => {
                let mut ptrdata: POINTER_TYPE_INFO = Default::default();

                ptrdata.r#type = PT_TOUCH;
                ptrdata.Anonymous.touchInfo.pointerInfo = ptrinfo;
                ptrdata.Anonymous.touchInfo.pointerInfo.pointerType = PT_TOUCH;
                ptrdata.Anonymous.touchInfo.pointerInfo.sourceDevice = HANDLE(self.synthetic_touch);
                ptrdata.Anonymous.touchInfo.touchFlags = 0;
                ptrdata.Anonymous.touchInfo.touchMask = 0;

                unsafe { InjectSyntheticPointerInput(self.synthetic_touch, &ptrdata, 1) };
            }
            #[allow(clippy::field_reassign_with_default)]
            PointerData::Stylus => {
                let mut ptrdata: POINTER_TYPE_INFO = Default::default();

                ptrdata.r#type = PT_PEN;
                ptrdata.Anonymous.penInfo.pointerInfo = ptrinfo;
                ptrdata.Anonymous.penInfo.pointerInfo.pointerType = PT_PEN;
                ptrdata.Anonymous.penInfo.pointerInfo.sourceDevice = HANDLE(self.synthetic_pen);
                ptrdata.Anonymous.penInfo.penFlags = 0; //TODO: Pen buttons
                ptrdata.Anonymous.penInfo.penMask = 0;

                unsafe { InjectSyntheticPointerInput(self.synthetic_pen, &ptrdata, 1) };
            }
            PointerData::Mouse { buttons } => {
                let buttons_pressed = buttons.difference(self.last_buttons);
                let buttons_released = self.last_buttons.difference(buttons);

                let mut inputs = vec![INPUT {
                    r#type: INPUT_MOUSE,
                    Anonymous: INPUT_0 {
                        mi: MOUSEINPUT {
                            dx: p.location.0 as i32,
                            dy: p.location.1 as i32,
                            mouseData: 0,
                            dwFlags: MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                }];

                let mut first = inputs.get_mut(0).unwrap();

                if p.location != self.last_mouse_position {
                    unsafe {
                        first.Anonymous.mi.dwFlags |= MOUSEEVENTF_MOVE;
                    }
                }

                if buttons_pressed.contains(MouseButtons::PRIMARY) {
                    unsafe {
                        first.Anonymous.mi.dwFlags |= MOUSEEVENTF_LEFTDOWN;
                    }
                }

                if buttons_released.contains(MouseButtons::PRIMARY) {
                    unsafe {
                        first.Anonymous.mi.dwFlags |= MOUSEEVENTF_LEFTUP;
                    }
                }

                if buttons_pressed.contains(MouseButtons::CONTEXT) {
                    unsafe {
                        first.Anonymous.mi.dwFlags |= MOUSEEVENTF_RIGHTDOWN;
                    }
                }

                if buttons_released.contains(MouseButtons::CONTEXT) {
                    unsafe {
                        first.Anonymous.mi.dwFlags |= MOUSEEVENTF_RIGHTUP;
                    }
                }

                if buttons_pressed.contains(MouseButtons::SCROLL) {
                    unsafe {
                        first.Anonymous.mi.dwFlags |= MOUSEEVENTF_MIDDLEDOWN;
                    }
                }

                if buttons_released.contains(MouseButtons::SCROLL) {
                    unsafe {
                        first.Anonymous.mi.dwFlags |= MOUSEEVENTF_MIDDLEUP;
                    }
                }

                // Windows's X-Button support is rather odd. We cannot
                // represent both back and forward being released at the same
                // time in one event. If we need to, then we have to issue two
                // events, just so that the second one can hold the ones we
                // pressed.
                if (buttons_pressed.contains(MouseButtons::BACK)
                    && buttons_released.contains(MouseButtons::FORWARD))
                    || (buttons_pressed.contains(MouseButtons::FORWARD)
                        && buttons_released.contains(MouseButtons::BACK))
                {
                    unsafe {
                        first.Anonymous.mi.dwFlags |= MOUSEEVENTF_XDOWN;
                        first.Anonymous.mi.mouseData |=
                            if buttons_pressed.contains(MouseButtons::BACK) {
                                XBUTTON1
                            } else {
                                0
                            } | if buttons_pressed.contains(MouseButtons::FORWARD) {
                                XBUTTON2
                            } else {
                                0
                            };
                    }

                    inputs.push(INPUT {
                        r#type: INPUT_MOUSE,
                        Anonymous: INPUT_0 {
                            mi: MOUSEINPUT {
                                dx: p.location.0 as i32,
                                dy: p.location.1 as i32,
                                mouseData: if buttons_released.contains(MouseButtons::BACK) {
                                    XBUTTON1
                                } else {
                                    0
                                } | if buttons_released.contains(MouseButtons::FORWARD) {
                                    XBUTTON2
                                } else {
                                    0
                                },
                                dwFlags: MOUSEEVENTF_ABSOLUTE
                                    | MOUSEEVENTF_VIRTUALDESK
                                    | MOUSEEVENTF_XUP,
                                time: 0,
                                dwExtraInfo: 0,
                            },
                        },
                    });
                } else if !buttons_pressed.is_empty() {
                    unsafe {
                        first.Anonymous.mi.dwFlags |= MOUSEEVENTF_XDOWN;
                        first.Anonymous.mi.mouseData |=
                            if buttons_pressed.contains(MouseButtons::BACK) {
                                XBUTTON1
                            } else {
                                0
                            } | if buttons_pressed.contains(MouseButtons::FORWARD) {
                                XBUTTON2
                            } else {
                                0
                            };
                    }
                } else if !buttons_released.is_empty() {
                    unsafe {
                        first.Anonymous.mi.dwFlags |= MOUSEEVENTF_XUP;
                        first.Anonymous.mi.mouseData |=
                            if buttons_released.contains(MouseButtons::BACK) {
                                XBUTTON1
                            } else {
                                0
                            } | if buttons_released.contains(MouseButtons::FORWARD) {
                                XBUTTON2
                            } else {
                                0
                            };
                    }
                }

                unsafe {
                    SendInput(
                        inputs.len() as u32,
                        inputs.as_ptr(),
                        size_of::<INPUT>() as i32,
                    );
                }

                self.last_buttons = buttons;
                self.last_mouse_position = p.location;
            }
        }
    }

    fn service_target(&mut self) {
        while let Ok(thread_msg) = self.recv.try_recv() {
            match thread_msg {
                TargetEvent::Pointer(p) => self.update_pointer(p),
                TargetEvent::Error(_) => {
                    //TODO: report error to user
                }
            }
        }
    }
}

fn main() {
    let mut msg: MSG = Default::default();
    let mut bret;

    let mut target = TargetServicer::setup();

    // Win32 event loop
    loop {
        bret = unsafe { GetMessageW(&mut msg as *mut MSG, None, 0, 0) };
        if !bret.as_bool() {
            break;
        }

        if msg.hwnd == 0 && msg.message == WM_APP {
            target.service_target();
        } else {
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}
