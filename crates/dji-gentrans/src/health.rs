use crate::private::*;

const SIZE: usize = 16;

/// Server to Robot
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GameRobotHP {
    ally_1: u16,
    ally_2: u16,
    ally_3: u16,
    ally_4: u16,
    _reserved: u16,
    ally_7: u16,
    ally_outpost: u16,
    ally_base: u16,
}

impl GameRobotHP {
    pub const fn get_ally1_hp(&self) -> u16 {
        self.ally_1
    }

    pub const fn get_ally2_hp(&self) -> u16 {
        self.ally_2
    }

    pub const fn get_ally3_hp(&self) -> u16 {
        self.ally_3
    }

    pub const fn get_ally4_hp(&self) -> u16 {
        self.ally_4
    }

    pub const fn get_ally7_hp(&self) -> u16 {
        self.ally_7
    }

    pub const fn get_outpost_hp(&self) -> u16 {
        self.ally_outpost
    }

    pub const fn get_base_hp(&self) -> u16 {
        self.ally_base
    }
}

impl Marshaler for GameRobotHP {
    const CMD_ID: u16 = 0x0003;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < SIZE {
            return Err(Error::BufferTooSmall {
                need: SIZE - dst.len(),
            });
        }

        dst[0..2].copy_from_slice(&self.ally_1.to_le_bytes());
        dst[2..4].copy_from_slice(&self.ally_2.to_le_bytes());
        dst[4..6].copy_from_slice(&self.ally_3.to_le_bytes());
        dst[6..8].copy_from_slice(&self.ally_4.to_le_bytes());
        dst[8..10].copy_from_slice(&self._reserved.to_le_bytes());
        dst[10..12].copy_from_slice(&self.ally_7.to_le_bytes());
        dst[12..14].copy_from_slice(&self.ally_outpost.to_le_bytes());
        dst[14..16].copy_from_slice(&self.ally_base.to_le_bytes());

        Ok(SIZE)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != SIZE {
            return Err(Error::InvalidDataLength { expected: SIZE });
        }

        let ally_1 = u16::from_le_bytes([raw[0], raw[1]]);
        let ally_2 = u16::from_le_bytes([raw[2], raw[3]]);
        let ally_3 = u16::from_le_bytes([raw[4], raw[5]]);
        let ally_4 = u16::from_le_bytes([raw[6], raw[7]]);
        let _reserved = u16::from_le_bytes([raw[8], raw[9]]);
        let ally_7 = u16::from_le_bytes([raw[10], raw[11]]);
        let ally_outpost = u16::from_le_bytes([raw[12], raw[13]]);
        let ally_base = u16::from_le_bytes([raw[14], raw[15]]);

        Ok(Self {
            ally_1,
            ally_2,
            ally_3,
            ally_4,
            _reserved,
            ally_7,
            ally_outpost,
            ally_base,
        })
    }
}

#[cfg(test)]
#[test]
fn test() {
    let status = GameRobotHP {
        ally_1: 1000,
        ally_2: 2000,
        ally_3: 3000,
        ally_4: 4000,
        _reserved: 0,
        ally_7: 7000,
        ally_outpost: 8000,
        ally_base: 9000,
    };

    let mut buf = [0u8; SIZE + 10];
    let sz = status.marshal(&mut buf).unwrap();
    assert_eq!(sz, SIZE);

    let decoded = GameRobotHP::unmarshal(&buf[..SIZE]).unwrap();
    assert_eq!(decoded.get_ally1_hp(), 1000);
    assert_eq!(decoded.get_ally2_hp(), 2000);
    assert_eq!(decoded.get_ally3_hp(), 3000);
    assert_eq!(decoded.get_ally4_hp(), 4000);
    assert_eq!(decoded.get_ally7_hp(), 7000);
    assert_eq!(decoded.get_outpost_hp(), 8000);
    assert_eq!(decoded.get_base_hp(), 9000);
}
