use crate::private::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Operate {
    NoOperation = 0,
    Add = 1,
    Modify = 2,
    Delete = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FigureType {
    Line = 0,
    Rectangle = 1,
    Circle = 2,
    Ellipse = 3,
    Arc = 4,
    FloatingPoint = 5,
    IntegerNumber = 6,
    Character = 7,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Color {
    SelfColor = 0,
    Yellow = 1,
    Green = 2,
    Orange = 3,
    Magenta = 4,
    Pink = 5,
    Cyan = 6,
    Black = 7,
    White = 8,
}

/// Robot to Client
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IaFigure {
    name: [u8; 3],
    operate_type: u8, // 3 bits
    figure_type: u8,  // 3 bits
    layer: u8,        // 4 bits
    color: u8,        // 4 bits
    details_a: u16,   // 9 bits
    details_b: u16,   // 9 bits
    width: u16,       // 10 bits
    start_x: u16,     // 11 bits
    start_y: u16,     // 11 bits
    details_c: u16,   // 10 bits
    details_d: u16,   // 11 bits
    details_e: u16,   // 11 bits
}

impl IaFigure {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 3],
            operate_type: 0,
            figure_type: 0,
            layer: 0,
            color: 0,
            details_a: 0,
            details_b: 0,
            width: 0,
            start_x: 0,
            start_y: 0,
            details_c: 0,
            details_d: 0,
            details_e: 0,
        }
    }
}

impl AsCommand<15> for IaFigure {
    fn as_command(&self) -> Command {
        Command::DeleteLayer
    }

    fn as_data(&self) -> [u8; 15] {
        let mut data = [0u8; 15];
        data[0..3].copy_from_slice(&self.name);
        // data[3..7].copy_from_slice(&self.operate1.to_le_bytes());
        // data[7..11].copy_from_slice(&self.operate2.to_le_bytes());
        // data[11..15].copy_from_slice(&self.operate3.to_le_bytes());
        data
    }
}

// #[cfg(test)]
// #[test]
// fn test() {
//     let delete_layer = DeleteLayer::new(DeleteType::DeleteLayer, 3);

//     assert_eq!(delete_layer.as_command(), Command::DeleteLayer);
//     assert_eq!(delete_layer.as_data(), [1, 3]);
// }
