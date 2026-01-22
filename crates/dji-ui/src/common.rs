use crate::private::*;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Command {
    DeleteLayer = 0x0100,
    DrawOneFigure = 0x0101,
    DrawTwoFigures = 0x0102,
    DrawFiveFigures = 0x0103,
    DrawSevenFigures = 0x0104,
    DrawCharacter = 0x0110,
}

pub trait AsCommand<const N: usize> {
    fn as_command(&self) -> Command;
    fn as_data(&self) -> [u8; N];
}

/// Robot to Client
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Interaction<const N: usize> {
    cmd_id: Command,
    sender: u16,
    receiver: u16,
    data: [u8; N],
}

impl<const N: usize> Interaction<N> {
    pub fn new(sender: u16, receiver: u16, option: impl AsCommand<N>) -> Self {
        let cmd_id = option.as_command();
        let data = option.as_data();
        Self {
            cmd_id,
            sender,
            receiver,
            data,
        }
    }
}

impl<const N: usize> Marshaler for Interaction<N> {
    const CMD_ID: u16 = 0x0301;

    fn marshal(&self, dst: &mut [u8]) -> Result<usize> {
        if dst.len() < N + 6 {
            return Err(Error::BufferTooSmall { need: N + 6 });
        }

        if N > 112 {
            return Err(Error::InputTooLarge { max: 112 });
        }

        let cmd_id = self.cmd_id as u16;

        dst[0..2].copy_from_slice(&cmd_id.to_le_bytes());
        dst[2..4].copy_from_slice(&self.sender.to_le_bytes());
        dst[4..6].copy_from_slice(&self.receiver.to_le_bytes());
        dst[6..6 + N].copy_from_slice(&self.data);

        Ok(N + 6)
    }

    fn unmarshal(raw: &[u8]) -> Result<Self> {
        if raw.len() != N + 6 {
            return Err(Error::InvalidDataLength {
                expected: N + 6 - raw.len(),
            });
        }

        let cmd_id = match u16::from_le_bytes([raw[0], raw[1]]) {
            0x0100 => Command::DeleteLayer,
            0x0101 => Command::DrawOneFigure,
            0x0102 => Command::DrawTwoFigures,
            0x0103 => Command::DrawFiveFigures,
            0x0104 => Command::DrawSevenFigures,
            0x0110 => Command::DrawCharacter,

            _ => {
                return Err(Error::DecodeError { at: 1 });
            }
        };

        let sender = u16::from_le_bytes([raw[2], raw[3]]);
        let receiver = u16::from_le_bytes([raw[4], raw[5]]);
        let mut data = [0u8; N];
        data.copy_from_slice(&raw[6..6 + N]);

        Ok(Self {
            cmd_id,
            sender,
            receiver,
            data,
        })
    }
}

#[cfg(test)]
#[test]
fn test() {
    let interaction: Interaction<4> = Interaction {
        cmd_id: Command::DrawOneFigure,
        sender: 0x1234,
        receiver: 0x5678,
        data: [1, 2, 3, 4],
    };

    let mut buf = [0u8; 10];
    let size = interaction.marshal(&mut buf).unwrap();
    assert_eq!(size, 10);

    let decoded = Interaction::<4>::unmarshal(&buf).unwrap();
    assert_eq!(decoded.cmd_id, interaction.cmd_id);
    assert_eq!(decoded.sender, interaction.sender);
    assert_eq!(decoded.receiver, interaction.receiver);
    assert_eq!(decoded.data, interaction.data);
}
