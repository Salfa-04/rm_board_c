#![cfg_attr(not(test), no_std)]

/// 0x0001 - Game Status
pub mod states;

/// 0x0002 - Game Result
pub mod result;

/// 0x0003 - Robot Health
pub mod health;

/// 0x0101 - Game Event
pub mod event;

/// 0x0104 - Referee Warning
pub mod warning;

/// 0x0105 - Dart Info
pub mod dart;

/// 0x0201 - Robot Status
pub mod status;

/// 0x0202 - Power Heat Data
pub mod heat;

/// 0x0203 - Robot Position
pub mod pos;

/// 0x0204 - Robot Buff Data
pub mod buff;

/// 0x0206 - Robot Hurt Data
pub mod hurt;

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

    assert_eq!(states::GameStatus::CMD_ID, 0x0001);
    assert_eq!(result::GameResult::CMD_ID, 0x0002);
    assert_eq!(health::GameRobotHP::CMD_ID, 0x0003);
    assert_eq!(event::GameEvent::CMD_ID, 0x0101);
    assert_eq!(warning::RefereeWarning::CMD_ID, 0x0104);
    assert_eq!(dart::DartInfo::CMD_ID, 0x0105);
    assert_eq!(status::RobotStatus::CMD_ID, 0x0201);
    assert_eq!(heat::PowerHeat::CMD_ID, 0x0202);
    assert_eq!(pos::RobotPos::CMD_ID, 0x0203);
    assert_eq!(buff::RobotBuff::CMD_ID, 0x0204);
    assert_eq!(hurt::HurtData::CMD_ID, 0x0206);
}
