use crate::{hal::can, system::*};

use utils::{
    MemCell,
    atomic::{AtomicBool, Ordering},
};

use can::{BufferedCan, Can, Fifo, RxBuf, TxBuf, filter::Mask32};

const TX_BUF_SIZE: usize = 25;
const RX_BUF_SIZE: usize = 10;

static CAN1_TX_BUF: MemCell<TxBuf<TX_BUF_SIZE>> = MemCell::uninit();
static CAN1_RX_BUF: MemCell<RxBuf<RX_BUF_SIZE>> = MemCell::uninit();
static CAN2_TX_BUF: MemCell<TxBuf<TX_BUF_SIZE>> = MemCell::uninit();
static CAN2_RX_BUF: MemCell<RxBuf<RX_BUF_SIZE>> = MemCell::uninit();

static UNBUFED_CAN1: MemCell<Can<'static>> = MemCell::uninit();
static UNBUFED_CAN2: MemCell<Can<'static>> = MemCell::uninit();

static BUFFERED_CAN1: MemCell<BufferedCan<'static, TX_BUF_SIZE, RX_BUF_SIZE>> = MemCell::uninit();
static BUFFERED_CAN2: MemCell<BufferedCan<'static, TX_BUF_SIZE, RX_BUF_SIZE>> = MemCell::uninit();

pub(super) async fn bxcan_init(
    p: CanSrc,
) -> (
    &'static BufferedCan<'static, TX_BUF_SIZE, RX_BUF_SIZE>,
    &'static BufferedCan<'static, TX_BUF_SIZE, RX_BUF_SIZE>,
) {
    let mut can1 = Can::new(p.can1_p, p.can1_rx, p.can1_tx, Irqs);
    let mut can2 = Can::new(p.can2_p, p.can2_rx, p.can2_tx, Irqs);

    // Only master can(1) has filters.
    can1.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all())
        .enable_bank(1, Fifo::Fifo1, Mask32::accept_all())
        .set_split(1);

    can1.modify_config()
        // .set_loopback(true)
        .set_silent(false)
        .set_bitrate(1_000_000)
        .set_automatic_retransmit(true);

    can2.modify_config()
        // .set_loopback(true)
        .set_silent(false)
        .set_bitrate(1_000_000)
        .set_automatic_retransmit(true);

    (can1.enable().await, can2.enable().await);

    // Safety: Only Called Once at Here
    unsafe { can_buffer_init(can1, can2) }
}

/// Safety: Can Only Be Called Once
unsafe fn can_buffer_init(
    can1: Can<'static>,
    can2: Can<'static>,
) -> (
    &'static BufferedCan<'static, TX_BUF_SIZE, RX_BUF_SIZE>,
    &'static BufferedCan<'static, TX_BUF_SIZE, RX_BUF_SIZE>,
) {
    static TAKEN: AtomicBool = AtomicBool::new(false);
    if TAKEN.swap(true, Ordering::AcqRel) {
        panic!("Can Buffers Have Already Been Taken");
    }

    // Safety: Static Can Instances are only initialized here.
    let can1 = unsafe { &mut *UNBUFED_CAN1.init(can1) };
    // Safety: Static Can Instances are only initialized here.
    let can2 = unsafe { &mut *UNBUFED_CAN2.init(can2) };

    // Safety: Static buffers are only initialized here.
    let can1_buffer = unsafe {
        (
            &mut *CAN1_TX_BUF.init(TxBuf::new()),
            &mut *CAN1_RX_BUF.init(RxBuf::new()),
        )
    };

    // Safety: Static buffers are only initialized here.
    let can2_buffer = unsafe {
        (
            &mut *CAN2_TX_BUF.init(TxBuf::new()),
            &mut *CAN2_RX_BUF.init(RxBuf::new()),
        )
    };

    // Safety: Static BufferedCan Instances are only initialized here.
    unsafe {
        (
            &*BUFFERED_CAN1.init(can1.buffered(can1_buffer.0, can1_buffer.1)),
            &*BUFFERED_CAN2.init(can2.buffered(can2_buffer.0, can2_buffer.1)),
        )
    }
}
