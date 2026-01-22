use crate::private::*;

const SIZE: usize = 3;

/// Server to Robot
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DartInfo {
    remaining_time: u8,
    dart_info: u16,
}

impl DartInfo {
    pub const fn remaining_time(&self) -> u8 {
        self.remaining_time
    }

    /// TODO: Need More Bits Info
    pub const fn dart_info(&self) -> u16 {
        self.dart_info
    }
}

impl Marshaler for DartInfo {
    const CMD_ID: u16 = 0x0105;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < SIZE {
            return Err(Error::BufferTooSmall { need: SIZE });
        }

        dst[0] = self.remaining_time;
        dst[1..3].copy_from_slice(&self.dart_info.to_le_bytes());

        Ok(SIZE)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != SIZE {
            return Err(Error::InvalidDataLength { expected: SIZE });
        }

        let remaining_time = raw[0];
        let dart_info = u16::from_le_bytes([raw[1], raw[2]]);

        Ok(DartInfo {
            remaining_time,
            dart_info,
        })
    }
}

#[cfg(test)]
#[test]
fn test() {
    let status = DartInfo {
        remaining_time: 120,
        dart_info: 0x3456,
    };

    let mut buf = [0u8; SIZE + 10];
    let sz = status.marshal(&mut buf).unwrap();
    assert_eq!(sz, SIZE);

    let decoded = DartInfo::unmarshal(&buf[..SIZE]).unwrap();
    assert_eq!(decoded.remaining_time, 120);
    assert_eq!(decoded.dart_info, 0x3456);
}
