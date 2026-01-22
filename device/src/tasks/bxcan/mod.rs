//!
//! # BxCAN Task
//!

use crate::system::*;

mod device;
mod init;
mod msger;

#[embassy_executor::task]
pub async fn task(s: embassy_executor::SendSpawner, p: CanSrc) {
    let (can1, can2) = init::bxcan_init(p).await;

    let (can1_tx, can1_rx) = (can1.writer(), can1.reader());
    let (can2_tx, can2_rx) = (can2.writer(), can2.reader());

    // s.must_spawn(msger::can1_snd::sender(can1_tx));
    s.must_spawn(msger::can1_rcv::receiver(can1_rx));

    // s.must_spawn(msger::can2_snd::sender(can2_tx));
    s.must_spawn(msger::can2_rcv::receiver(can2_rx));
}
