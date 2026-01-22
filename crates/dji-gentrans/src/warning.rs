use crate::private::*;

const SIZE: usize = 3;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Level {
    YellowCardBoth = 1,
    YellowCard = 2,
    RedCard = 3,
    Loss = 4,
}

/// Server to Robot
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RefereeWarning {
    level: Level,
    robot_id: u8,
    count: u8,
}

impl RefereeWarning {
    pub const fn level(&self) -> Level {
        self.level
    }

    pub const fn robot_id(&self) -> u8 {
        self.robot_id
    }

    pub const fn count(&self) -> u8 {
        self.count
    }
}

impl Marshaler for RefereeWarning {
    const CMD_ID: u16 = 0x0104;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < SIZE {
            return Err(Error::BufferTooSmall { need: SIZE });
        }

        dst[0] = self.level as u8;
        dst[1] = self.robot_id;
        dst[2] = self.count;

        Ok(SIZE)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != SIZE {
            return Err(Error::InvalidDataLength { expected: SIZE });
        }

        let level = match raw[0] {
            1 => Level::YellowCardBoth,
            2 => Level::YellowCard,
            3 => Level::RedCard,
            4 => Level::Loss,

            _ => return Err(Error::DecodeError { at: 0 }),
        };

        let robot_id = raw[1];
        let count = raw[2];

        Ok(RefereeWarning {
            level,
            robot_id,
            count,
        })
    }
}

#[cfg(test)]
#[test]
fn test() {
    let warning = RefereeWarning {
        level: Level::RedCard,
        robot_id: 5,
        count: 2,
    };

    let mut buf = [0u8; SIZE + 10];
    let sz = warning.marshal(&mut buf).unwrap();
    assert_eq!(sz, SIZE);

    let decoded = RefereeWarning::unmarshal(&buf[..SIZE]).unwrap();
    assert_eq!(decoded.level, Level::RedCard);
    assert_eq!(decoded.robot_id, 5);
    assert_eq!(decoded.count, 2);
}
