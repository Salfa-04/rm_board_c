use crate::private::*;

/// Custom Robot Controller to Controlled Robot
/// 30 Bytes Max
/// frequency: 30Hz
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Custom2Robot {}

impl Marshaler for Custom2Robot {
    const CMD_ID: u16 = 0x0302;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        todo!()
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        todo!()
    }
}
