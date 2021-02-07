pub mod coply {
    use std::{cell::RefCell, rc::Rc};

    pub const CHUNK_SIZE: i32 = 128;

    pub type ChunkDataType = Vec<u8>;
    type ChunkRef = Rc<RefCell<Chunk>>;
    pub type ChunkOpt = Option<ChunkRef>;

    #[derive(Debug, Clone)]
    pub enum ChunkData {
        Data(ChunkDataType),
        Empty
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
        pub fn set_option(chunk: Chunk) -> ChunkOpt {
            Option::Some(Chunk::set_reference(chunk))
        }
        pub fn set_reference(chunk: Chunk) -> ChunkRef {
            Rc::new(RefCell::new(chunk))
        }
        pub fn get_reference(chunk_opt: ChunkOpt) -> ChunkRef {
            chunk_opt.clone().unwrap()
        }
        pub fn get_from_option(chunk_opt: ChunkOpt) -> Chunk {
            Chunk::get_from_reference(chunk_opt.clone().unwrap())
        }
        pub fn get_from_reference(chunk_ref: ChunkRef) -> Chunk {
            chunk_ref
            .clone()
            .try_borrow_mut()
            .expect("Could not get the borrow mut")
            .to_owned()
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
}