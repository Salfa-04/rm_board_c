//!
//! DaMiao Motor Interface
//!
//! # Recommended Configuration
//!
//! - Baud Rate: 1Mbps
//! - Control: PV Mode
//! - CAN Timeout: 2000 (100ms/50us)
//! - Max Velocity: 2 rad/s
//! - P_MAX: 12.56 rad
//!
//! # DM-J4310-2EC-V1.2 Specifications
//! - V_MAX: 23 rad/s
//! - T_MAX: 10 Nm
//! - Acceleration: +2 rad/s²
//! - Deceleration: -2 rad/s²
//!
//! # DM-J8006-2EC-V1.1 Specifications
//! - V_MAX: 20 rad/s
//! - T_MAX: 20 Nm
//! - Acceleration: +1.5 rad/s²
//! - Deceleration: -1.5 rad/s²
//!
//! # DM-J10010L-2EC Specifications
//! - V_MAX: 12.5 rad/s
//! - T_MAX: 120 Nm
//! - Acceleration: +1 rad/s²
//! - Deceleration: -1 rad/s²
//!

use super::private::*;

#[repr(u8)]
#[derive(defmt::Format, Debug, PartialEq)]
pub enum DaMiaoState {
    Disabled = 0x0,
    Enabled = 0x1,
    OverVoltage = 0x8,
    UnderVoltage = 0x9,
    OverCurrent = 0xA,
    OverTempMOS = 0xB,
    OverTempROT = 0xC,
    ConnectionLost = 0xD,
    OverLoad = 0xE,

    /// ID Must be in 0~15
    IncorrectID = 0xFF,
}

pub trait DaMiaoConfig {
    /// Motor Master ID
    const MSTID: u16;

    /// Motor CAN ID
    const CANID: u16;

    const P_MAX: f32;
    const V_MAX: f32;
    const T_MAX: f32;

    /// Maximum Position in rad
    const MAX_POS: f32 = PI;
    /// Minimum Position in rad
    const MIN_POS: f32 = -PI;

    /// Assertion to ensure valid position range
    const __: () = assert!(Self::MAX_POS > Self::MIN_POS);
}

pub trait DaMiaoMotor: DaMiaoConfig {
    /// Get the raw 64-bit data from the motor
    fn get_raw(&self) -> u64;

    /// Update the motor data from a byte slice
    fn update(&self, src: &Frame) -> bool;

    /// CAN ID of the motor (0~15)
    fn id(&self) -> u8 {
        (self.get_raw() & 0xF) as u8
    }

    /// Motor Error Status
    fn sta(&self) -> DaMiaoState {
        let err = ((self.get_raw() >> 4) & 0x0F) as u8;
        match err {
            0x0 => DaMiaoState::Disabled,
            0x1 => DaMiaoState::Enabled,
            0x8 => DaMiaoState::OverVoltage,
            0x9 => DaMiaoState::UnderVoltage,
            0xA => DaMiaoState::OverCurrent,
            0xB => DaMiaoState::OverTempMOS,
            0xC => DaMiaoState::OverTempROT,
            0xD => DaMiaoState::ConnectionLost,
            0xE => DaMiaoState::OverLoad,
            _ => DaMiaoState::IncorrectID,
        }
    }

    /// Position in rad
    fn pos(&self) -> f32 {
        let pos = ((self.get_raw() >> 8) & 0xFFFF) as u16;
        // (pos.swap_bytes() as f32 / 32767. - 1.) * const { Self::P_MAX * 360. / TAU }
        (pos.swap_bytes() as f32 / 32767. - 1.) * Self::P_MAX
    }

    /// Velocity in rad/s
    fn vel(&self) -> f32 {
        let vel = ((self.get_raw() >> 24) & 0xF0FF) as u16;
        let vel = (vel & 0xFF) << 4 | (vel >> 12) & 0x0F;
        // (vel as f32 / 2047. - 1.) * const { Self::V_MAX * 60. / TAU }
        (vel as f32 / 2047. - 1.) * Self::V_MAX
    }

    /// Torque in Nm
    fn tor(&self) -> f32 {
        let tor = ((self.get_raw() >> 32) & 0xFF0F) as u16;
        let tor = (tor >> 8) & 0xFF | (tor & 0x0F) << 8;
        (tor as f32 / 2047. - 1.) * Self::T_MAX
    }

    /// MOS Temperature in Celsius
    fn temp_mos(&self) -> f32 {
        ((self.get_raw() >> 48) & 0xFF) as f32
    }

    /// Rotor Temperature in Celsius
    fn temp_rot(&self) -> f32 {
        ((self.get_raw() >> 56) & 0xFF) as f32
    }
}

pub trait DaMiaoCtrl: DaMiaoConfig {
    /// Get Motor Feedback Frame
    fn get_fb(&self) -> Frame {
        let canid_l = (Self::CANID & 0xFF) as u8;
        let canid_h = ((Self::CANID >> 8) & 0x7) as u8;
        Frame::new_standard(
            0x7FF, // Broadcast ID
            &[canid_l, canid_h, 0xCC, 0],
        )
        .expect("Invalid CAN ID!")
    }

    /// Enable the motor with pv mode
    fn enable(&self) -> Frame {
        Frame::new_standard(
            0x100 + Self::CANID, // PV Mode ID
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFC],
        )
        .expect("Invalid CAN ID!")
    }

    /// Disable the motor from pv mode
    fn disable(&self) -> Frame {
        Frame::new_standard(
            0x100 + Self::CANID, // PV Mode ID
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFD],
        )
        .expect("Invalid CAN ID!")
    }

    /// Clear Error with pv mode
    fn clr_err(&self) -> Frame {
        Frame::new_standard(
            0x100 + Self::CANID, // PV Mode ID
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFB],
        )
        .expect("Invalid CAN ID!")
    }

    /// Set Position (rad) and Velocity (rad/s)
    fn set_pv(&self, p: f32, v: f32) -> Frame {
        let p = p.clamp(Self::MIN_POS, Self::MAX_POS);
        let pos = p.to_le_bytes();
        let vel = v.abs().to_le_bytes();
        Frame::new_standard(
            0x100 + Self::CANID, // PV Mode ID
            &[
                pos[0], pos[1], pos[2], pos[3], vel[0], vel[1], vel[2], vel[3],
            ],
        )
        .expect("Invalid CAN ID!")
    }

    /// Enable the motor with torque mode
    fn enable_torque(&self) -> Frame {
        Frame::new_standard(
            Self::CANID, // Torque Mode ID
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFC],
        )
        .expect("Invalid CAN ID!")
    }

    /// Disable the motor from torque mode
    fn disable_torque(&self) -> Frame {
        Frame::new_standard(
            Self::CANID, // Torque Mode ID
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFD],
        )
        .expect("Invalid CAN ID!")
    }

    /// Clear Error with torque mode
    fn clr_err_torque(&self) -> Frame {
        Frame::new_standard(
            Self::CANID, // PV Mode ID
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFB],
        )
        .expect("Invalid CAN ID!")
    }

    /// Set Torque (Nm)
    fn set_torque(&self, t: f32) -> Frame {
        let t = t.clamp(-Self::T_MAX, Self::T_MAX);
        let t = ((t / Self::T_MAX + 1.) * (0x7FF as f32)) as u16 & 0xFFF;
        let t = t.to_be_bytes();
        Frame::new_standard(
            Self::CANID, // Torque Mode ID
            &[0x7F, 0xFF, 0x7F, 0xF0, 0x00, 0x00, t[0] & 0xF, t[1]],
        )
        .expect("Invalid CAN ID!")
    }
}

#[macro_export]
macro_rules! damiao {
    ($name:ident) => {
        #[non_exhaustive]
        pub struct $name(AtomicU64);

        impl $name {
            #[inline]
            pub fn get() -> &'static Self {
                static INS: $name = $name(AtomicU64::new(0));
                &INS
            }
        }

        impl DaMiaoCtrl for $name {}

        impl DaMiaoMotor for $name {
            fn get_raw(&self) -> u64 {
                self.0.load(Order)
            }

            fn update(&self, src: &Frame) -> bool {
                let data = src.data();
                if (data.len() != 8) || (data[1] == 0x00 && data[2] == 0x55) {
                    return false;
                }

                let raw = u64::from_le_bytes(
                    // Safety: the length has been checked
                    data.try_into().unwrap(),
                );

                self.0.store(raw, Order);
                true
            }
        }

        impl defmt::Format for $name {
            fn format(&self, fmt: defmt::Formatter) {
                defmt::write!(
                    fmt,
                    "{} {{ id: 0x{:X}, sta: {}, pos: {} rad, vel: {} rad/s, tor: {} Nm, temp_mos: {}°C, temp_rot: {}°C }}",
                    stringify!($name),
                    self.id(), self.sta(),
                    self.pos(), self.vel(), self.tor(),
                    self.temp_mos(), self.temp_rot()
                );
            }
        }

    };
}
