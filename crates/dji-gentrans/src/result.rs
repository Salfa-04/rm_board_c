use crate::private::*;

const SIZE: usize = 1;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Winner {
    Draw = 0,
    Red = 1,
    Blue = 2,
}

/// Server to Robot
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GameResult {
    winner: Winner,
}

impl GameResult {
    pub const fn winner(&self) -> Winner {
        self.winner
    }
}

impl Marshaler for GameResult {
    const CMD_ID: u16 = 0x0002;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < SIZE {
            return Err(Error::BufferTooSmall { need: SIZE });
        }

        dst[0] = self.winner as u8;

        Ok(SIZE)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != SIZE {
            return Err(Error::InvalidDataLength { expected: SIZE });
        }

        let winner = match raw[0] {
            0 => Winner::Draw,
            1 => Winner::Red,
            2 => Winner::Blue,

            _ => return Err(Error::DecodeError { at: 0 }),
        };

        Ok(GameResult { winner })
    }
}

#[cfg(test)]
#[test]
fn test() {
    let status = GameResult {
        winner: Winner::Blue,
    };

    let mut buf = [0u8; SIZE + 10];
    let sz = status.marshal(&mut buf).unwrap();
    assert_eq!(sz, SIZE);

    let decoded = GameResult::unmarshal(&buf[..SIZE]).unwrap();
    assert_eq!(decoded.winner, Winner::Blue);
}
