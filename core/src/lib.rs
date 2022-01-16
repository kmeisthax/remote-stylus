mod error;
mod process;
mod protocol;

pub use error::Error;
pub use process::{source, target};
pub use protocol::{MouseButtons, Point, PointerData, PointerEvent, StreamAction};
