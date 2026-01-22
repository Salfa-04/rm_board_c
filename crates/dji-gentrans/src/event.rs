use crate::private::*;

const SIZE: usize = 4;

/// Server to Robot
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GameEvent {
    event_data: u32,
}

impl GameEvent {
    /// TODO: Need More specific event data decoding
    pub const fn event_data(&self) -> u32 {
        self.event_data
    }
}

impl Marshaler for GameEvent {
    const CMD_ID: u16 = 0x0101;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < SIZE {
            return Err(Error::BufferTooSmall { need: SIZE });
        }

        dst[0..4].copy_from_slice(&self.event_data.to_le_bytes());

        Ok(SIZE)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != SIZE {
            return Err(Error::InvalidDataLength { expected: SIZE });
        }

        let event_data = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);

        Ok(GameEvent { event_data })
    }
}

#[cfg(test)]
#[test]
fn test() {
    let status = GameEvent {
        event_data: 0x12345678,
    };

    let mut buf = [0u8; SIZE + 10];
    let sz = status.marshal(&mut buf).unwrap();
    assert_eq!(sz, SIZE);

    let decoded = GameEvent::unmarshal(&buf[..SIZE]).unwrap();
    assert_eq!(decoded.event_data, 0x12345678);
}
