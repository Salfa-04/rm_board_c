use crate::private::*;

const SIZE: usize = 1;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reason {
    HitByProjectile = 0,
    ModuleOffline = 1,
    StruckByImpact = 5,
}

/// Main Ctrl Module to Robot
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HurtData {
    armor_id: u8,
    deduction_reason: Reason,
}

impl HurtData {
    pub const fn armor_id(&self) -> u8 {
        self.armor_id
    }

    pub const fn deduction_reason(&self) -> Reason {
        self.deduction_reason
    }
}

impl Marshaler for HurtData {
    const CMD_ID: u16 = 0x0206;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < SIZE {
            return Err(Error::BufferTooSmall { need: SIZE });
        }

        dst[0] = (self.armor_id & 0xF) | (((self.deduction_reason as u8) & 0xF) << 4);

        Ok(SIZE)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != SIZE {
            return Err(Error::InvalidDataLength { expected: SIZE });
        }

        let armor_id = raw[0] & 0x0F;
        let deduction_reason = match (raw[0] >> 4) & 0xF {
            0 => Reason::HitByProjectile,
            1 => Reason::ModuleOffline,
            5 => Reason::StruckByImpact,

            _ => return Err(Error::DecodeError { at: 0 }),
        };

        Ok(HurtData {
            armor_id,
            deduction_reason,
        })
    }
}

#[cfg(test)]
#[test]
fn test() {
    let hurt = HurtData {
        armor_id: 3,
        deduction_reason: Reason::ModuleOffline,
    };

    let mut buf = [0u8; SIZE];
    let sz = hurt.marshal(&mut buf).unwrap();
    assert_eq!(sz, SIZE);
    assert_eq!(buf[0], 0x13);

    let hurt2 = HurtData::unmarshal(&buf).unwrap();
    assert_eq!(hurt2.armor_id(), 3);
    assert_eq!(hurt2.deduction_reason(), Reason::ModuleOffline);
}
