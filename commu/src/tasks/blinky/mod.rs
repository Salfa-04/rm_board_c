//!
//! # Blinky Task
//!

use crate::{hal, system::*};

use hal::peripherals::TIM5;
use hal::{gpio::OutputType, time::khz, timer};
use timer::low_level::CountingMode::EdgeAlignedUp;
use timer::simple_pwm::SimplePwmChannel;
use timer::simple_pwm::{PwmPin, SimplePwm};

const FPS: f32 = 1000.;
const SPEED: u16 = 1;

/// # HUE to RGB Conversion
/// Converts a hue value (0-1535) to RGB values (0-255).
fn color_wheel(hue: u16) -> (u8, u8, u8) {
    let x = (hue & 0xFF) as u8;
    match hue >> 8 {
        0 => (255, x, 0),       // Red -> Yellow
        1 => (255 - x, 255, 0), // Yellow -> Green
        2 => (0, 255, x),       // Green -> Cyan
        3 => (0, 255 - x, 255), // Cyan -> Blue
        4 => (x, 0, 255),       // Blue -> Magenta
        _ => (255, 0, 255 - x), // Magenta -> Red
    }
}

#[embassy_executor::task]
pub async fn task(p: BlinkySrc) -> ! {
    let mut t = utils::init_ticker!(const { 1000. / FPS } as u64);

    let (mut r, mut g, mut b) = init(p);
    (r.enable(), g.enable(), b.enable());

    let mut hue: u16 = 0;

    loop {
        let (rv, gv, bv) = color_wheel(hue);
        r.set_duty_cycle_fraction(rv as u32, 255);
        g.set_duty_cycle_fraction(gv as u32, 255);
        b.set_duty_cycle_fraction(bv as u32, 255);
        hue = (hue + SPEED) % 1536;

        t.next().await
    }
}

fn init(
    p: BlinkySrc,
) -> (
    SimplePwmChannel<'static, TIM5>,
    SimplePwmChannel<'static, TIM5>,
    SimplePwmChannel<'static, TIM5>,
) {
    let b = PwmPin::new(p.led_b, OutputType::PushPull);
    let g = PwmPin::new(p.led_g, OutputType::PushPull);
    let r = PwmPin::new(p.led_r, OutputType::PushPull);

    let chn = SimplePwm::new(
        p.tim_p,
        Some(b),
        Some(g),
        Some(r),
        None,
        khz(1),
        EdgeAlignedUp,
    )
    .split();

    (chn.ch3, chn.ch2, chn.ch1)
}
