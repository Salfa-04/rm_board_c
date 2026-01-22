use crate::private::*;

const SIZE: usize = 8;

/// Server to Robot
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RobotBuff {
    recovery_rate: u8,
    colling_value: u16,
    defence_rate: u8,
    vulnerablity_rate: u8,
    attack_rate: u16,
    remain_energy: u8,
}

impl RobotBuff {
    pub const fn recovery_rate(&self) -> u8 {
        self.recovery_rate
    }

    pub const fn colling_value(&self) -> u16 {
        self.colling_value
    }

    pub const fn defence_rate(&self) -> u8 {
        self.defence_rate
    }

    pub const fn vulnerablity_rate(&self) -> u8 {
        self.vulnerablity_rate
    }

    pub const fn attack_rate(&self) -> u16 {
        self.attack_rate
    }

    /// TODO: Need More Bits Info
    pub const fn remain_energy(&self) -> u8 {
        self.remain_energy
    }
}

impl Marshaler for RobotBuff {
    const CMD_ID: u16 = 0x0204;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < SIZE {
            return Err(Error::BufferTooSmall { need: SIZE });
        }

        dst[0] = self.recovery_rate;
        dst[1..3].copy_from_slice(&self.colling_value.to_le_bytes());
        dst[3] = self.defence_rate;
        dst[4] = self.vulnerablity_rate;
        dst[5..7].copy_from_slice(&self.attack_rate.to_le_bytes());
        dst[7] = self.remain_energy;

        Ok(SIZE)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != SIZE {
            return Err(Error::InvalidDataLength { expected: SIZE });
        }

        let recovery_rate = raw[0];
        let colling_value = u16::from_le_bytes([raw[1], raw[2]]);
        let defence_rate = raw[3];
        let vulnerablity_rate = raw[4];
        let attack_rate = u16::from_le_bytes([raw[5], raw[6]]);
        let remain_energy = raw[7];

        Ok(RobotBuff {
            recovery_rate,
            colling_value,
            defence_rate,
            vulnerablity_rate,
            attack_rate,
            remain_energy,
        })
    }
}

#[cfg(test)]
#[test]
fn test() {
    let buff = RobotBuff {
        recovery_rate: 10,
        colling_value: 200,
        defence_rate: 5,
        vulnerablity_rate: 3,
        attack_rate: 1500,
        remain_energy: 80,
    };

    let mut buf = [0u8; SIZE];
    let sz = buff.marshal(&mut buf).unwrap();
    assert_eq!(sz, SIZE);

    let buff2 = RobotBuff::unmarshal(&buf).unwrap();
    assert_eq!(buff2.recovery_rate(), 10);
    assert_eq!(buff2.colling_value(), 200);
    assert_eq!(buff2.defence_rate(), 5);
    assert_eq!(buff2.vulnerablity_rate(), 3);
    assert_eq!(buff2.attack_rate(), 1500);
    assert_eq!(buff2.remain_energy(), 80);
}
