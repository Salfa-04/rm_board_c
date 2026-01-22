//!
//! System Initialization
//!
//! # Definitions
//!
//! - `__pre_init`: Early Initialization Assembly Routine
//! - `sys_init`: System Initialization Function
//!

use crate::prelude::{hal, ll};
use hal::{Config, Peripherals, init, rcc, time::mhz};
use ll::Peripherals as CorePeripherals;

// `__pre_init` will be called before main
core::arch::global_asm! {
    ".global __pre_init",
    ".type __pre_init, %function",
    ".thumb_func",
    "__pre_init:",

    // // Copy ITCM from FLASH to ITCM
    // "ldr r0, =__sitcm
    //  ldr r1, =__eitcm
    //  ldr r2, =__siitcm
    //  0:
    //  cmp r0, r1
    //  bhs 1f
    //  ldmia r2!, {{r3, r4}}
    //  stmia r0!, {{r3, r4}}
    //  b 0b
    //  1:",

    // return
    "bx lr",
}

///
/// System Initialization Function
///
/// This function initializes the system peripherals and clocks.
///
pub fn sys_init() -> (CorePeripherals, Peripherals) {
    defmt::debug!("System Initialization...");

    let core = match CorePeripherals::take() {
        Some(x) => x,
        None => panic!("{}: Can Be Called Only Once!!!", file!()),
    };

    let peripherals = {
        let mut config = Config::default();
        let rcc = &mut config.rcc;

        init(config) // SysClock = xMHz
    };

    (core, peripherals)
}
