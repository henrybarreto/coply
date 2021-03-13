use crate::ChunkDataType;

#[derive(Debug, Clone)]
pub enum ChunkData {
    Data(ChunkDataType),
    Empty,
}
impl ChunkData {
    pub fn unwrap(&self) -> Result<ChunkDataType, ()> {
        if let ChunkData::Data(data) = self.clone() {
            Ok(data)
        } else {
            Err(())
        }
    }
}
