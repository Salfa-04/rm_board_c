//!
//! # System Interrupts
//!

use super::private::*;

bind_interrupts! {
    pub struct Irqs {
        // LPUART1 => hal::usart::InterruptHandler<peripherals::LPUART1>;

        CAN1_TX => hal::can::TxInterruptHandler<peripherals::CAN1>;
        CAN1_RX0 => hal::can::Rx0InterruptHandler<peripherals::CAN1>;
        CAN1_RX1 => hal::can::Rx1InterruptHandler<peripherals::CAN1>;
        CAN1_SCE => hal::can::SceInterruptHandler<peripherals::CAN1>;

        CAN2_TX => hal::can::TxInterruptHandler<peripherals::CAN2>;
        CAN2_RX0 => hal::can::Rx0InterruptHandler<peripherals::CAN2>;
        CAN2_RX1 => hal::can::Rx1InterruptHandler<peripherals::CAN2>;
        CAN2_SCE => hal::can::SceInterruptHandler<peripherals::CAN2>;
    }
}
