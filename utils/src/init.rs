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
        config.enable_debug_during_sleep = true;

        let rcc = &mut config.rcc;

        rcc.hsi = false; // HSI = 16MHz
        rcc.hse = Some(rcc::Hse {
            freq: mhz(12),
            mode: rcc::HseMode::Oscillator,
        });

        rcc.pll_src = rcc::PllSource::HSE; // HSE = 12MHz
        rcc.pll = Some(rcc::Pll {
            prediv: rcc::PllPreDiv::DIV6,   //   2MHz
            mul: rcc::PllMul::MUL168,       // 336MHz
            divp: Some(rcc::PllPDiv::DIV2), // 168MHz
            divq: Some(rcc::PllQDiv::DIV7), //  48MHz
            divr: None,                     // Not used
        });

        rcc.plli2s = None; // Not used

        rcc.sys = rcc::Sysclk::PLL1_P; // 168MHz
        rcc.ahb_pre = rcc::AHBPrescaler::DIV1; // 168MHz
        rcc.apb1_pre = rcc::APBPrescaler::DIV4; // 42MHz
        rcc.apb2_pre = rcc::APBPrescaler::DIV2; // 84MHz

        rcc.ls = rcc::LsConfig::default_lsi(); // LSI = 32KHz
        rcc.mux.clk48sel = rcc::mux::Clk48sel::PLL1_Q; // 48MHz
        rcc.mux.rtcsel = rcc::mux::Rtcsel::DISABLE; // Disabled
        rcc.mux.sdiosel = rcc::mux::Sdiosel::CLK48; // 48MHz

        init(config) // SysClock = 168MHz
    };

    (core, peripherals)
}
