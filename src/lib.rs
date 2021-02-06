pub mod coply {
    use core::panic;
    use std::{cell::RefCell, rc::Rc};

    pub const CHUNK_SIZE: i32 = 128;

    pub type ChunkDataType = [u8];
    type ChunkRef = Rc<RefCell<Chunk>>;
    type ChunkOpt = Option<ChunkRef>;

    #[derive(Debug, Clone)]
    pub enum ChunkData {
        Data(Box<ChunkDataType>),
        Empty
    }
    impl ChunkData {
        pub fn unwrap(&self) -> Box<ChunkDataType> {
            if let ChunkData::Data(data) = self.clone() {
                data
            } else {
                panic!("Could not unwrap ChunkData");
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Chunk {
        pub data: ChunkData,
        pub next: ChunkOpt
    }
    impl Chunk {
        pub fn new(data: ChunkData) -> Self {
            Chunk {
                data,
                next: ChunkOpt::None
            }
        }
    }

    pub struct Buffer {
        pub chunks: ChunkOpt
    }

    impl Buffer {
        pub fn new() -> Self {
            Buffer {
                chunks: Option::None
            }
        }
        pub fn add_chunk(&mut self, data: ChunkData) -> Chunk {
            let chunk = Chunk::new(data);
            if let Option::None = self.chunks.clone() {
                let chunk_clone = chunk.clone();
                self.chunks = Option::Some(Rc::new(RefCell::new(chunk_clone)));
            } else {
                let chunks_from_buffer = self.chunks.clone().unwrap();
                let mut chunk_clone = chunk.clone();
                chunk_clone.next = Some(chunks_from_buffer);
                self.chunks = Option::Some(Rc::new(RefCell::new(chunk_clone)));
            }
            self.chunks
            .clone()
            .unwrap()
            .try_borrow()
            .unwrap()
            .to_owned() // study
        }
    }
}