#![cfg_attr(not(test), no_std)]

pub use common::*;

mod common;

pub mod delete_layer;
pub mod draw_figure;

mod private {
    #[allow(unused_imports)]
    #[cfg(feature = "defmt")]
    pub use ::defmt::{debug, error, info, trace, warn};

    pub use crate::common::{AsCommand, Command};
    pub use dji_frame::{Error, Marshaler, Result};
}

#[cfg(test)]
#[test]
fn test_command_id() {
    use crate::private::Marshaler;

    assert_eq!(common::Interaction::<0>::CMD_ID, 0x0301);
}
