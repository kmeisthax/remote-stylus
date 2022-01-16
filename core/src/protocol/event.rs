use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/// A location on the virtual desktop.
///
/// Point coordinates are relative to the target display's physical pixel
/// coordinates; e.g. a 2048x1536 display accepts points from (0,0) to
/// (2048,1536) regardless of the pixel size of the input.
#[derive(Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Point(pub u16, pub u16);

/// The type of pointing device.
///
/// Pointer device type should be reported as specifically as possible.
/// Specifically, event sources are expected to check for and bypass
/// compatibility features that might report, say, a finger touch as a click.
/// Event targets are likewise expected to inject events as faithfully as
/// possible, while allowing the target platform's compatibility events to be
/// triggered normally.
///
/// In the event that a particular compatibility method *cannot* be bypassed,
/// event sources should report what their platforms do report. For example, if
/// a platform supports stylus input, but that stylus generates finger inputs,
/// then it is fine to report them as finger inputs.
///
/// In the event that a particular platform does not support a given input
/// type, event targets must implement reasonable compatibility measures that
/// make sense for the given platform. For example, a platform that does not
/// support touch input should treat touches as clicks.
enum PointerType {
    /// A relative pointer method in which the user moves a small puck or spins
    /// a ball to produce input.
    ///
    /// Generally speaking, mice generate only one pointer event stream at a
    /// time.
    Mouse,

    /// An absolute pointer method in which the user presses one or more
    /// fingers against the display surface to produce input.
    ///
    /// Fingers are capable of generating multiple pointer event streams, and
    /// optional information about finger size, pressure, and so on.
    Finger,

    /// An absolute pointer method in which the user uses a stick to draw on
    /// the device.
    ///
    /// Generally speaking, styluses generate only one pointer event stream at
    /// a time. But they also typically include pressure and tilt information.
    Stylus,
}

bitflags! {
    /// List of mouse buttons that are currently being pressed.
    ///
    /// The names of the buttons are reported here without reference to
    /// physical position, as these buttons will be in different positions
    /// depending on what kind of mouse is in use and if the user is left- or
    /// right-handed. Event sources must report buttons in this abstract way so
    /// that the source platform's chirality preference is taken into account.
    #[derive(Serialize, Deserialize, Default)]
    pub struct MouseButtons: u8 {
        /// The primary mouse button.
        ///
        /// For right-handed users, this is typically the left mouse button.
        const PRIMARY = 0b00000001;

        /// The secondary/context mouse button.
        ///
        /// For right-handed users, this is typically the right mouse button.
        const CONTEXT = 0b00000010;

        /// The middle mouse button or scroll wheel click.
        const SCROLL  = 0b00000100;

        /// The mouse's back button.
        const BACK    = 0b00001000;

        /// The mouse's forward button.
        const FORWARD = 0b00010000;
    }
}

/// Auxiliary pointer data provided by a given type of pointing device.
#[derive(Serialize, Deserialize)]
pub enum PointerData {
    /// Pointer data specific to `Mouse` pointers.
    Mouse {
        /// What buttons are currently pressed on the mouse.
        buttons: MouseButtons,
    },

    /// Pointer data specific to `Finger` pointers.
    Finger,

    /// Pointer data specific to `Stylus` pointers.
    Stylus,
}

impl From<PointerData> for PointerType {
    fn from(pdata: PointerData) -> PointerType {
        match pdata {
            PointerData::Mouse { .. } => PointerType::Mouse,
            PointerData::Finger => PointerType::Finger,
            PointerData::Stylus => PointerType::Stylus,
        }
    }
}

/// What action is being taken on the pointer event stream.
#[derive(Serialize, Deserialize)]
pub enum StreamAction {
    /// A new pointer was added to the event stream.
    ///
    /// The stream ID for this pointer event must not currently be in use by
    /// another pointer of this type.
    Start,

    /// Existing pointer data was updated.
    ///
    /// e.g. a pointer was moved, mouse button pressed, stylus tilted, etc
    Update,

    /// A pointer was removed from the event stream.
    ///
    /// This may also contain a final pointer data update.
    ///
    /// Once a stream has been ended, it's stream ID may be reused by a
    /// subsequent `Start` action.
    End,

    /// A pointer was cancelled from the event stream.
    ///
    /// This is equivalent to ending a pointer stream, with the added
    /// implication that it was ended due to platform-specific actions, such as
    /// an alert window occluding a tracked finger.
    ///
    /// If a target does not support pointer cancellation it should be treated
    /// as an `End` action instead.
    Cancel,
}

/// An event generated by a pointer-type input device.
#[derive(Serialize, Deserialize)]
pub struct PointerEvent {
    /// What we're doing to the given pointer.
    pub stream_action: StreamAction,

    /// The pointer event stream ID.
    ///
    /// This is used to identify multiple pointers operating simultaneously.
    /// Stream IDs must remain stable and unique while a particular pointer is
    /// in use.
    ///
    /// Furthermore, streams are always one particular pointer type. To prevent
    /// inadvertent pointer type changes, stream IDs are specific to the type
    /// of pointer being reported on (e.g. finger #1 is not the same as stylus
    /// #1).
    pub stream: u8,

    /// The current location of the pointer.
    pub location: Point,

    /// Any data specific to the pointing device.
    pub data: PointerData,
}
