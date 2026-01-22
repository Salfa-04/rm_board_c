//!
//! # PicTrans Task
//!

use crate::{hal::usart, system::*};

use dji_frame::*;
use usart::{Config, DataBits, Parity, StopBits, UartRx};
use utils::heapless::Vec;

#[embassy_executor::task]
pub async fn task(p: Uart3pSrc) -> ! {
    let mut config = Config::default();
    config.baudrate = 921600;
    config.data_bits = DataBits::DataBits8;
    config.parity = Parity::ParityNone;
    config.stop_bits = StopBits::STOP1;

    // Safety: Config is valid, so Unwrap is safe.
    let mut pt = UartRx::new(p.uart_p, Irqs, p.uart_rx, p.dma_rx, config).unwrap();

    let mut buffer = [0u8; 64];
    let mut data: _ = Vec::<u8, 128>::new();

    loop {
        match pt.read_until_idle(&mut buffer).await {
            Ok(x) if x > 0 => {
                if let Err(_) = data.extend_from_slice(&buffer[..x]) {
                    defmt::warn!("RC Data Overflow, clearing buffer");
                    data.clear();
                    continue;
                }

                let s = data_process::<_, 5>(&mut data);
                defmt::info!("RC Data: {:X}", s);
            }

            Ok(_) => {
                // No data received
            }

            Err(e) => {
                defmt::error!("RC Read Error: {:?}", e);
            }
        };
    }
}

#[derive(Debug, defmt::Format)]
struct CustomRobotData<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Marshaler for CustomRobotData<N> {
    const CMD_ID: u16 = 0x0302;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < N {
            return Err(Error::BufferTooSmall {
                need: N - dst.len(),
            });
        }

        dst[..N].copy_from_slice(&self.data);
        Ok(N)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != N {
            return Err(Error::InvalidDataLength { expected: N });
        }

        let mut data = [0u8; N];
        data.copy_from_slice(&raw[..N]);
        Ok(CustomRobotData { data })
    }
}

fn data_process<const N: usize, const R: usize>(
    src: &mut Vec<u8, N>,
) -> Option<CustomRobotData<R>> {
    let msger: Messager<DjiValidator> = Messager::new(0);

    match msger.unpack(src) {
        Ok((x, size)) => {
            // defmt::info!("Parsed RC Data: {:X}", x);
            let id = x.cmd_id();
            let seq = x.sequence();
            let msg = match id {
                CustomRobotData::<R>::CMD_ID => CustomRobotData::<R>::unmarshal(x.payload()).ok(),
                _ => {
                    defmt::warn!("Unknown RC Data CMD ID: {}", id);
                    None
                }
            };

            src.drain(..size);
            msg
        }

        Err(e) => {
            src.drain(..e.skip());
            None
        }
    }
}
