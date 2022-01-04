//! On-the-wire packet format

use serde::{Deserialize, Serialize};

mod event;

#[derive(Serialize, Deserialize)]
pub enum Packet {
    PointerEvent(event::PointerEvent),
}
