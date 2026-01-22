use super::private::*;

#[embassy_executor::task]
pub async fn sender(mut can: BufferedCanSender) -> ! {
    let mut t = utils::init_ticker!(1, ms);

    loop {
        match SysMode::get() {
            SysMode::Error => {}

            SysMode::Boot => {}

            SysMode::Normal => {}
        }

        can.write(Frame::new_standard(0x08, &[1, 2, 3, 4]).unwrap())
            .await;

        t.next().await
    }
}
