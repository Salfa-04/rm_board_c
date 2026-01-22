//!
//! # Dji Motor Interface
//!

use super::private::*;

///
/// # DJI M3508 Motor
///
/// - Output Shaft No-Load Speed: 482rpm
/// - Output Torque Constant: 0.3Nm/A
/// - Reduction Ratio: 3591:187
/// - Maximum Allowable Winding Temperature: 125℃
///
/// # DJI GM6020 Motor
///
/// - Output Shaft No-Load Speed: 320rpm
/// - Output Torque Constant: 0.741Nm/A
/// - Reduction Ratio: 1:1
/// - Maximum Allowable Winding Temperature: 125℃
///
/// **Viewed from the Shaft End, the Motor Rotates CCW**
///
pub trait DjiMotor {
    /// Motor Master ID
    const MSTID: u16;

    /// Torque Constant in Nm/A
    const TORQUE_CONSTANT: f32;
    /// Reduction Ratio (from Motor to Output Shaft)
    const REDUCTION_RATIO: f32;

    /// Get the raw 64-bit data from the motor
    fn get_raw(&self) -> u64;

    /// Update the motor data from a byte slice
    fn update(&self, src: &Frame) -> bool;

    /// Position in Degrees
    fn pos(&self) -> f32 {
        let pos = (self.get_raw() & 0xFFFF) as u16;
        pos.swap_bytes() as f32 * const { 360. / 8192. }
    }

    /// Velocity in RPM
    fn vel(&self) -> f32 {
        let vel = ((self.get_raw() >> 16) & 0xFFFF) as i16;
        vel.swap_bytes() as f32 * const { 1. / Self::REDUCTION_RATIO }
    }

    /// Torque in Nm
    fn tor(&self) -> f32 {
        let tor = ((self.get_raw() >> 32) & 0xFFFF) as i16;
        tor.swap_bytes() as f32 * const { 20. / 16384. * Self::TORQUE_CONSTANT }
    }

    /// Temperature in Celsius
    fn temp(&self) -> u8 {
        ((self.get_raw() >> 48) & 0xFF) as u8
    }
}

pub trait DjiCtrl {
    /// Control Command ID
    const CANID: u16;

    /// Set the current for four motors (A, B, C, D)
    fn set_cur(current: (i16, i16, i16, i16)) -> Frame {
        let crt_1 = current.0.to_be_bytes();
        let crt_2 = current.1.to_be_bytes();
        let crt_3 = current.2.to_be_bytes();
        let crt_4 = current.3.to_be_bytes();

        Frame::new_standard(
            Self::CANID,
            &[
                // Safety: all slices are of length 2
                crt_1[0], crt_1[1], // Motor A, for id 1 (+4)
                crt_2[0], crt_2[1], // Motor B, for id 2 (+4)
                crt_3[0], crt_3[1], // Motor C, for id 3 (+4)
                crt_4[0], crt_4[1], // Motor D, for id 4 (+4)
            ],
        )
        .unwrap()
    }
}

#[macro_export]
macro_rules! dji_motor {
    ($name:ident, $mstid:expr, 3508) => {
        $crate::dji_motor!($name, $mstid, 0.3, 3591.0 / 187.0);
    };

    ($name:ident, $mstid:expr, 6020) => {
        $crate::dji_motor!($name, $mstid, 0.741, 1.0);
    };

    ($name:ident, $mstid:expr, $torque:expr, $reduction:expr) => {
        #[non_exhaustive]
        pub struct $name(AtomicU64);

        impl $name {
            #[inline]
            pub fn get() -> &'static Self {
                static INSTANCE: $name = $name(AtomicU64::new(0));
                &INSTANCE
            }
        }

        impl DjiMotor for $name {
            const MSTID: u16 = $mstid;

            const TORQUE_CONSTANT: f32 = $torque;
            const REDUCTION_RATIO: f32 = $reduction;

            fn get_raw(&self) -> u64 {
                self.0.load(Order)
            }

            fn update(&self, src: &Frame) -> bool {
                let data = src.data();

                if data.len() != 8 {
                    return false;
                }

                let raw = u64::from_le_bytes(
                    // safe: length has been checked
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
                    "{} {{ pos: {}°, vel: {} RPM, tor:{} Nm, temp: {}°C }}",
                    stringify!($name),
                    self.pos(),
                    self.vel(),
                    self.tor(),
                    self.temp()
                );
            }
        }
    };
}
