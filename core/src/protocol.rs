//! On-the-wire packet format

use serde::{Deserialize, Serialize};

mod event;

pub use event::{MouseButtons, Point, PointerData, PointerEvent, StreamAction};

/// Any message to be sent between event sources and targets.
#[derive(Serialize, Deserialize)]
pub enum Message {
    /// Message sent by an event source to connect to an event target.
    SourceConnectTarget,

    /// Message sent by a target back to the source to acknowledge connection.
    ///
    /// Sources should not send other messages until and unless they recieve
    /// a `TargetAcknowledgeSource`.
    TargetAcknowledgeSource,

    /// Message sent by an event source to report a pointer event occurring.
    SourcePointerEvent(PointerEvent),
}
