use crate::private::*;

const SIZE: usize = 12;

/// Main Ctrl Module to Robot
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RobotPos {
    x: f32,
    y: f32,
    z: f32,
}

impl RobotPos {
    pub const fn pos_x(&self) -> f32 {
        self.x
    }

    pub const fn pos_y(&self) -> f32 {
        self.y
    }

    /// 0 towards north
    pub const fn angle(&self) -> f32 {
        self.z
    }
}

impl Marshaler for RobotPos {
    const CMD_ID: u16 = 0x0203;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < SIZE {
            return Err(Error::BufferTooSmall { need: SIZE });
        }

        dst[0..4].copy_from_slice(&self.x.to_le_bytes());
        dst[4..8].copy_from_slice(&self.y.to_le_bytes());
        dst[8..12].copy_from_slice(&self.z.to_le_bytes());

        Ok(SIZE)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != SIZE {
            return Err(Error::InvalidDataLength { expected: SIZE });
        }

        let x = f32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
        let y = f32::from_le_bytes([raw[4], raw[5], raw[6], raw[7]]);
        let z = f32::from_le_bytes([raw[8], raw[9], raw[10], raw[11]]);

        Ok(RobotPos { x, y, z })
    }
}

#[cfg(test)]
#[test]
fn test() {
    let pos = RobotPos {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let mut buf = [0u8; SIZE];
    let sz = pos.marshal(&mut buf).unwrap();
    assert_eq!(sz, SIZE);

    let pos2 = RobotPos::unmarshal(&buf).unwrap();
    assert_eq!(pos2.pos_x(), 1.0);
    assert_eq!(pos2.pos_y(), 2.0);
    assert_eq!(pos2.angle(), 3.0);
}
