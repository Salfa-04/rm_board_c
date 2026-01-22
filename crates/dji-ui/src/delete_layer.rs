use crate::private::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeleteType {
    NoOperation = 0,
    DeleteLayer = 1,
    DeleteAllLayers = 2,
}

/// Robot to Client
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeleteLayer {
    delete_type: DeleteType,
    layer: u8,
}

impl DeleteLayer {
    pub fn new(delete_type: DeleteType, layer: u8) -> Self {
        Self { delete_type, layer }
    }
}

impl AsCommand<2> for DeleteLayer {
    fn as_command(&self) -> Command {
        Command::DeleteLayer
    }

    fn as_data(&self) -> [u8; 2] {
        [self.delete_type as u8, self.layer]
    }
}

#[cfg(test)]
#[test]
fn test() {
    let delete_layer = DeleteLayer::new(DeleteType::DeleteLayer, 3);

    assert_eq!(delete_layer.as_command(), Command::DeleteLayer);
    assert_eq!(delete_layer.as_data(), [1, 3]);
}
