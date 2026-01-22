//!
//! # CAN Bus Messenger
//!

pub mod can1_rcv;
pub mod can1_snd;
pub mod can2_rcv;
pub mod can2_snd;

mod private {
    pub use super::super::{device::*, *};
    use crate::{hal::can, sync};

    pub use can::{BufferedCanReceiver, BufferedCanSender, Frame, Id};
    // pub use raw::CriticalSectionRawMutex as RM;
    // pub use sync::{blocking_mutex::raw, signal::Signal};
}
