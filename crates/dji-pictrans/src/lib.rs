#![cfg_attr(not(test), no_std)]

pub use custom::Custom2Robot;
pub use remote::RemoteControl;

/// 0x0302 - Custom to Robot
mod custom;

/// 0x0304 - Remote Control
mod remote;

mod private {
    #[allow(unused_imports)]
    #[cfg(feature = "defmt")]
    pub use ::defmt::{debug, error, info, trace, warn};

    pub use dji_frame::{Error, Marshaler, Result};
}

#[cfg(test)]
#[test]
fn test_command_id() {
    use crate::private::Marshaler;

    assert_eq!(custom::Custom2Robot::CMD_ID, 0x0302);
    assert_eq!(remote::RemoteControl::CMD_ID, 0x0304);
}
