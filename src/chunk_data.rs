use crate::ChunkDataType;

/// This enumeration stores options of each can be, i can be either a ChunkDataType or Empty.
#[derive(Debug, Clone)]
pub enum ChunkData {
    Data(ChunkDataType),
    Empty,
}
impl ChunkData {
    /// Unwrap the enumeration into a ChunkDataType
    pub fn unwrap(&self) -> Result<ChunkDataType, ()> {
        if let ChunkData::Data(data) = self.clone() {
            Ok(data)
        } else {
            Err(())
        }
    }
}
