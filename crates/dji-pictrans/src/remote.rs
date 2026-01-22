use crate::private::*;

/// Keyboard to Controlled Robot
/// frequency: 30Hz
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RemoteControl {
    mouse_x: i16,
    mouse_y: i16,
    mouse_z: i16,
    left_button: bool,
    right_button: bool,
    keyboard_v: u16,
    _reserved: u16,
}

impl RemoteControl {
    pub const fn mouse_vx(&self) -> i16 {
        self.mouse_x
    }

    pub const fn mouse_vy(&self) -> i16 {
        self.mouse_y
    }

    pub const fn mouse_vz(&self) -> i16 {
        self.mouse_z
    }

    pub const fn left_button_pressed(&self) -> bool {
        self.left_button
    }

    pub const fn right_button_pressed(&self) -> bool {
        self.right_button
    }
}

impl RemoteControl {
    pub const fn keyboard_w(&self) -> bool {
        (self.keyboard_v & (1 << 0)) != 0
    }

    pub const fn keyboard_s(&self) -> bool {
        (self.keyboard_v & (1 << 1)) != 0
    }

    pub const fn keyboard_a(&self) -> bool {
        (self.keyboard_v & (1 << 2)) != 0
    }

    pub const fn keyboard_d(&self) -> bool {
        (self.keyboard_v & (1 << 3)) != 0
    }

    pub const fn keyboard_shift(&self) -> bool {
        (self.keyboard_v & (1 << 4)) != 0
    }

    pub const fn keyboard_ctrl(&self) -> bool {
        (self.keyboard_v & (1 << 5)) != 0
    }

    pub const fn keyboard_q(&self) -> bool {
        (self.keyboard_v & (1 << 6)) != 0
    }

    pub const fn keyboard_e(&self) -> bool {
        (self.keyboard_v & (1 << 7)) != 0
    }

    pub const fn keyboard_r(&self) -> bool {
        (self.keyboard_v & (1 << 8)) != 0
    }

    pub const fn keyboard_f(&self) -> bool {
        (self.keyboard_v & (1 << 9)) != 0
    }

    pub const fn keyboard_g(&self) -> bool {
        (self.keyboard_v & (1 << 10)) != 0
    }

    pub const fn keyboard_z(&self) -> bool {
        (self.keyboard_v & (1 << 11)) != 0
    }

    pub const fn keyboard_x(&self) -> bool {
        (self.keyboard_v & (1 << 12)) != 0
    }

    pub const fn keyboard_c(&self) -> bool {
        (self.keyboard_v & (1 << 13)) != 0
    }

    pub const fn keyboard_v(&self) -> bool {
        (self.keyboard_v & (1 << 14)) != 0
    }

    pub const fn keyboard_b(&self) -> bool {
        (self.keyboard_v & (1 << 15)) != 0
    }
}

impl Marshaler for RemoteControl {
    const CMD_ID: u16 = 0x0304;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < 12 {
            return Err(Error::BufferTooSmall {
                need: 12 - dst.len(),
            });
        }

        dst[0..2].copy_from_slice(&self.mouse_x.to_le_bytes());
        dst[2..4].copy_from_slice(&self.mouse_y.to_le_bytes());
        dst[4..6].copy_from_slice(&self.mouse_z.to_le_bytes());
        dst[6] = if self.left_button { 1 } else { 0 };
        dst[7] = if self.right_button { 1 } else { 0 };
        dst[8..10].copy_from_slice(&self.keyboard_v.to_le_bytes());
        dst[10..12].copy_from_slice(&self._reserved.to_le_bytes());

        Ok(12)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != 12 {
            return Err(Error::InvalidDataLength { expected: 12 });
        }

        let mouse_x = i16::from_le_bytes([raw[0], raw[1]]);
        let mouse_y = i16::from_le_bytes([raw[2], raw[3]]);
        let mouse_z = i16::from_le_bytes([raw[4], raw[5]]);
        let left_button = raw[6] != 0;
        let right_button = raw[7] != 0;
        let keyboard_v = u16::from_le_bytes([raw[8], raw[9]]);
        let _reserved = u16::from_le_bytes([raw[10], raw[11]]);

        Ok(RemoteControl {
            mouse_x,
            mouse_y,
            mouse_z,
            left_button,
            right_button,
            keyboard_v,
            _reserved,
        })
    }
}

#[cfg(test)]
#[test]
fn test() {
    let rc = RemoteControl {
        mouse_x: 100,
        mouse_y: -100,
        mouse_z: 50,
        left_button: true,
        right_button: false,
        keyboard_v: 0b0000_0000_0001_1010,
        _reserved: 0,
    };

    let mut buf = [0u8; 20];
    let sz = rc.marshal(&mut buf).unwrap();
    assert_eq!(sz, 12);

    let decoded = RemoteControl::unmarshal(&buf[..12]).unwrap();
    assert_eq!(decoded.mouse_vx(), 100);
    assert_eq!(decoded.mouse_vy(), -100);
    assert_eq!(decoded.mouse_vz(), 50);
    assert_eq!(decoded.left_button_pressed(), true);
    assert_eq!(decoded.right_button_pressed(), false);
    assert_eq!(decoded.keyboard_w(), false);
    assert_eq!(decoded.keyboard_s(), true);
    assert_eq!(decoded.keyboard_a(), false);
    assert_eq!(decoded.keyboard_d(), true);
    assert_eq!(decoded.keyboard_shift(), true);
    assert_eq!(decoded.keyboard_ctrl(), false);
    assert_eq!(decoded.keyboard_q(), false);
    assert_eq!(decoded.keyboard_e(), false);
    assert_eq!(decoded.keyboard_r(), false);
    assert_eq!(decoded.keyboard_f(), false);
    assert_eq!(decoded.keyboard_g(), false);
    assert_eq!(decoded.keyboard_z(), false);
    assert_eq!(decoded.keyboard_x(), false);
    assert_eq!(decoded.keyboard_c(), false);
    assert_eq!(decoded.keyboard_v(), false);
    assert_eq!(decoded.keyboard_b(), false);
}
