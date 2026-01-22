use super::private::*;

#[embassy_executor::task]
pub async fn sender(mut can: BufferedCanSender) -> ! {
    let mut t = utils::init_ticker!(2, ms);

    let mut mode_last = SysMode::get();

    loop {
        let mode = SysMode::get();

        if mode != mode_last {
            match (&mode_last, &mode) {
                (_, SysMode::Normal) => {}

                _ => {}
            }
        }

        match mode {
            SysMode::Error => {}

            SysMode::Boot => {}

            SysMode::Normal => {}
        }

        mode_last = mode;

        t.next().await
    }
}
