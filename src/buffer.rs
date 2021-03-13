use crate::chunk::Chunk;
use crate::chunk_data::ChunkData;
use crate::ChunkOpt;

#[derive(Debug, Clone)]
pub struct Buffer {
    pub chunks: ChunkOpt,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            chunks: Option::None,
        }
    }
    pub fn add_data(&mut self, data: ChunkData) -> Chunk {
        let chunk = Chunk::new(data);
        if let Option::None = self.chunks.clone() {
            let chunk_clone = chunk.clone();
            self.chunks = Chunk::set_option(chunk_clone);
        } else {
            let chunks_from_buffer = self.chunks.clone();
            let mut chunk_clone = chunk.clone();
            chunk_clone.next = chunks_from_buffer;
            self.chunks = Chunk::set_option(chunk_clone);
        }
        Chunk::get_from_option(self.chunks.clone())
    }
    pub fn join_data(&self) -> Vec<u8> {
        let chunk = Chunk::get_from_option(self.chunks.clone());
        let mut actual_chunk = chunk.clone();
        let mut all_data: Vec<u8> = vec![];
        loop {
            if let ChunkData::Data(data) = actual_chunk.data.clone() {
                all_data = [data, all_data].concat();
                if let ChunkOpt::Some(_next) = actual_chunk.next.clone() {
                    actual_chunk = Chunk::get_from_option(actual_chunk.next);
                } else {
                    break;
                }
            }
        }
        all_data
    }
}
