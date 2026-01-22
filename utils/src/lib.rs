//!
//! Utility Functions and Types for Embedded Development.
//!

#![no_std]
#![no_main]
#![allow(unused_imports)]

use ::defmt_rtt as _;
use ::panic_probe as _;

mod cell;
mod init;
mod macros;

pub use cell::MemCell;
pub use init::sys_init;

/// Re-exports of `Cortex-M` Assembly Instructions
pub use prelude::ll::asm;
/// Re-exports of `Cortex-M` Peripheral Types
pub use prelude::ll::peripheral;
/// Re-exports of `Timer`
pub use prelude::time::Timer as T;

/// Re-exports of Atomic Types
pub mod atomic {
    pub use ::portable_atomic::*;
}

/// Re-exports of `Heapless` Crate
pub mod heapless {
    pub use ::heapless::*;
}

/// Re-exports of `Strum` Crate
pub mod strum {
    pub use ::strum::*;
}

/// Preludes for Commonly Used Crates
pub mod prelude {
    pub use ::cortex_m as ll; // Low Level
    pub use ::embassy_futures as ef; // Futures
    pub use ::embassy_stm32 as hal; // HAL
    pub use ::embassy_sync as sync; // Sync
    pub use ::embassy_time as time; // Time
}

/// Defmt Panic Handler
#[::defmt::panic_handler]
fn soft_panic() -> ! {
    ::panic_probe::hard_fault()
}
