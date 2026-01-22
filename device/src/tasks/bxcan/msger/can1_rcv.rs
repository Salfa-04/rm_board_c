use super::private::*;

#[inline]
fn wrap_angle(radians: f32) -> f32 {
    use core::f32::consts::{PI, TAU};
    let mut x = (radians + PI) % TAU;
    if x < 0. {
        x += TAU
    }
    x - PI
}

#[embassy_executor::task]
pub async fn receiver(can: BufferedCanReceiver) -> ! {
    let dmotor = DMotor::get();
    // let mut angle = Angle::new(0f32.to_radians());

    loop {
        match can.receive().await.map(|x| x.frame) {
            Ok(f) => match f.id() {
                Id::Standard(id) => match id.as_raw() {
                    DMotor::MSTID => {
                        if dmotor.update(&f) {
                            let pos = dmotor.pos();
                            // angle.update(pos.to_radians());
                            // defmt::info!("{} => {:?}", pos, angle);

                            let angle = wrap_angle((pos - 170.).to_radians());
                            defmt::info!("{}° =>: {}°", pos, angle.to_degrees());
                        } else {
                            defmt::warn!("Failed to parse DMotor frame: {:?}", f);
                        }
                    }

                    _ => {
                        defmt::info!("Received S frame: {:?}", f);
                    }
                },

                Id::Extended(id) => match *id {
                    _ => {
                        defmt::info!("Received E frame: {:?}", f);
                    }
                },
            },

            Err(e) => defmt::warn!("CAN Error: {}", e),
        }
    }
}
