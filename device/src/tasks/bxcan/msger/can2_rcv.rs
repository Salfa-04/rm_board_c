use super::private::*;

#[embassy_executor::task]
pub async fn receiver(can: BufferedCanReceiver) -> ! {
    loop {
        match can.receive().await.map(|x| x.frame) {
            Ok(f) => match f.id() {
                Id::Standard(id) => match id.as_raw() {
                    _ => {
                        defmt::info!("Received S frame: {:?}", f);
                    }
                },

                Id::Extended(id) => match (id.as_raw() & 0xFF) as u16 {
                    _ => {
                        defmt::info!("Received E frame: {:?}", f);
                    }
                },
            },

            Err(e) => defmt::warn!("CAN Error: {}", e),
        }
    }
}
