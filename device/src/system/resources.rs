//!
//! # System Resources
//!
//! ## Reserved Resources
//!

use super::private::*;

assign_resources! {
    /// for `Blinky` task.
    blinky: BlinkySrc {
        rng_p: RNG,
        tim_p: TIM5,
        led_r: PH12,
        led_g: PH11,
        led_b: PH10,
    }

    /// Controls the 5V Power.
    power: PowerSrc {
        tim_p: TIM3,
        ch3_pin: PC8,
    }

    usb: UsbSrc {
        usb_p: USB_OTG_FS,
        usb_dm: PA11,
        usb_dp: PA12,
        usb_otg: PA10,
    }

    CfgIO: CfgIOSrc {
        i2c_p: I2C2,
        i2c_scl: PF1,
        i2c_sda: PF0,
        i2c_dma_rx: DMA1_CH6,
        i2c_dma_tx: DMA1_CH2,
        sp2: SPI2,
        spi_cs: PB12,
        spi_clk: PB13,
        spi_miso: PB14,
        spi_mosi: PB15,
        spi_dma_rx: DMA1_CH3,
        spi_dma_tx: DMA1_CH4,
    }

    uart3p: Uart3pSrc {
        uart_p: USART6,
        uart_tx: PG14,
        uart_rx: PG9,
        dma_rx: DMA2_CH2,
        dma_tx: DMA2_CH6,
    }

    uart4p: Uart4pSrc {
        uart_p: USART1,
        uart_tx: PA9,
        uart_rx: PB7,
        dma_rx: DMA2_CH5,
        dma_tx: DMA2_CH7,
    }

    can: CanSrc {
        can1_p: CAN1,
        can1_tx: PD1,
        can1_rx: PD0,
        can2_p: CAN2,
        can2_tx: PB6,
        can2_rx: PB5,
    }

    pwm: PwmSrc {
        tim1_p: TIM1,
        ch1_pin1: PE9,
        ch2_pin1: PE11,
        ch3_pin1: PE13,
        ch4_pin1: PE14,
        tim8_p: TIM8,
        ch1_pin8: PC6,
        ch2_pin8: PI6,
        ch3_pin8: PI7,
    }

    fpc: FpcSrc {
        // todo: fix this
    }

    buzzer: BuzzerSrc {
        tim_p: TIM4,
        ch3: PD14,
    }

    bat: BatSrc {
        // todo: fix this
    }

    imu: ImuSrc {
        // todo: fix this
    }
}
