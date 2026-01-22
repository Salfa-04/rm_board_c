use crate::private::*;

const SIZE: usize = 11;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GameType {
    RMUC = 1,
    RMUT = 2,
    RMUA = 3,
    RMUL3V3 = 4,
    RMUL1V1 = 5,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GameProgress {
    NotStarted = 0,
    PrePared = 1,
    SelfCheck = 2,
    CountDown5s = 3,
    InProgress = 4,
    Calculating = 5,
}

/// Server to Robot
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GameStatus {
    game_type: GameType,
    game_progress: GameProgress,
    remaining_time_s: u16,
    unix_timestamp: u64,
}

impl GameStatus {
    pub const fn game_type(&self) -> GameType {
        self.game_type
    }

    pub const fn game_progress(&self) -> GameProgress {
        self.game_progress
    }

    pub const fn remaining_time_s(&self) -> u16 {
        self.remaining_time_s
    }

    pub const fn unix_timestamp(&self) -> u64 {
        self.unix_timestamp
    }
}

impl Marshaler for GameStatus {
    const CMD_ID: u16 = 0x0001;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < SIZE {
            return Err(Error::BufferTooSmall {
                need: SIZE - dst.len(),
            });
        }

        dst[0] = (self.game_type as u8) & 0xF | ((self.game_progress as u8) & 0xF) << 4;
        dst[1..3].copy_from_slice(&self.remaining_time_s.to_le_bytes());
        dst[3..11].copy_from_slice(&self.unix_timestamp.to_le_bytes());

        Ok(SIZE)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != SIZE {
            return Err(Error::InvalidDataLength { expected: SIZE });
        }

        let game_type = match raw[0] & 0xF {
            1 => GameType::RMUC,
            2 => GameType::RMUT,
            3 => GameType::RMUA,
            4 => GameType::RMUL3V3,
            5 => GameType::RMUL1V1,

            _ => return Err(Error::DecodeError { at: 0 }),
        };

        let game_progress = match (raw[0] >> 4) & 0xF {
            0 => GameProgress::NotStarted,
            1 => GameProgress::PrePared,
            2 => GameProgress::SelfCheck,
            3 => GameProgress::CountDown5s,
            4 => GameProgress::InProgress,
            5 => GameProgress::Calculating,

            _ => return Err(Error::DecodeError { at: 0 }),
        };

        let remaining_time_s = u16::from_le_bytes([raw[1], raw[2]]);
        let unix_timestamp = u64::from_le_bytes([
            raw[3], raw[4], raw[5], raw[6], raw[7], raw[8], raw[9], raw[10],
        ]);

        Ok(GameStatus {
            game_type,
            game_progress,
            remaining_time_s,
            unix_timestamp,
        })
    }
}

#[cfg(test)]
#[test]
fn test() {
    let status = GameStatus {
        game_type: GameType::RMUA,
        game_progress: GameProgress::InProgress,
        remaining_time_s: 1234,
        unix_timestamp: 1672531199,
    };

    let mut buf = [0u8; SIZE + 10];
    let sz = status.marshal(&mut buf).unwrap();
    assert_eq!(sz, SIZE);

    let decoded = GameStatus::unmarshal(&buf[..SIZE]).unwrap();
    assert_eq!(decoded.game_type(), GameType::RMUA);
    assert_eq!(decoded.game_progress(), GameProgress::InProgress);
    assert_eq!(decoded.remaining_time_s(), 1234);
    assert_eq!(decoded.unix_timestamp(), 1672531199);
}
