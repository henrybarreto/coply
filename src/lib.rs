pub mod coply {
    use std::{
        cell::RefCell,
        fs::{metadata, File},
        io::Read,
        rc::Rc,
    };

    pub const CHUNK_SIZE: u8 = 128;
    pub const CHUNKS_BY_BUFFER: u8 = 4;

    pub type ChunkDataType = Vec<u8>;
    type ChunkRef = Rc<RefCell<Chunk>>;
    pub type ChunkOpt = Option<ChunkRef>;

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

    #[derive(Debug, Clone)]
    pub struct Chunk {
        pub data: ChunkData,
        pub next: ChunkOpt,
    }
    impl Chunk {
        pub fn new(data: ChunkData) -> Self {
            Chunk {
                data,
                next: ChunkOpt::None,
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

    #[derive(Debug, Clone)]
    pub struct Manager {
        pub buffers: Vec<Buffer>,
    }

    impl Manager {
        pub fn new() -> Self {
            Manager {
                buffers: Vec::new(),
            }
        }
        pub fn read(&mut self, file_path: &str) -> Vec<Buffer> {
            let file = Rc::new(File::open(file_path.clone()).expect("Could not open the file"));
            let file_info =
                metadata(file_path.clone()).expect("Could not get the metada from the file");
            let file_size = file_info.len();

            let interation = Interation::new(file_size, CHUNK_SIZE);

            let buffer = RefCell::new(Buffer::new());
            interation.iter(
                |bytes| {
                    let mut data: Vec<u8> = vec![0; bytes as usize];
                    file.try_clone()
                        .expect("Could not clone the file")
                        .read(&mut data[..])
                        .expect("Could not read from the file");
                    buffer
                        .try_borrow_mut()
                        .expect("Could not borrow the buffer to a normal interation")
                        .add_data(ChunkData::Data(data));
                },
                |last_bytes| {
                    let mut data: Vec<u8> = vec![0; last_bytes as usize];
                    file.try_clone()
                        .expect("Could not clone the file")
                        .read(&mut data[..])
                        .expect("Could not read from the file");
                    buffer
                        .try_borrow_mut()
                        .expect("Could not borrow the buffer to the last interation")
                        .add_data(ChunkData::Data(data));
                },
            );
            self.buffers
                .push(buffer.try_borrow_mut().unwrap().to_owned());

            self.buffers.clone()
        }
        pub fn write() {}
    }
    pub struct Interation {
        pub count: u32,
        pub bytes: u8,
        pub last_bytes: u8,
    }
    impl Interation {
        pub fn new(file_size: u64, chunk_size: u8) -> Self {
            if (file_size % CHUNK_SIZE as u64) == 0 {
                Interation {
                    count: (file_size / chunk_size as u64) as u32,
                    bytes: chunk_size,
                    last_bytes: chunk_size,
                }
            } else {
                Interation {
                    count: ((file_size / chunk_size as u64) + 1) as u32,
                    bytes: chunk_size,
                    last_bytes: (file_size % chunk_size as u64) as u8,
                }
            }
        }
        pub fn iter<N, L>(&self, mut iter: N, iter_last: L)
        where
            N: FnMut(u8) -> (),
            L: Fn(u8) -> (),
        {
            for i in 1..self.count + 1 {
                if self.is_last(i) {
                    iter_last(self.last_bytes);
                } else {
                    iter(self.bytes);
                }
            }
        }
        pub fn is_last(&self, interation: u32) -> bool {
            if self.count == interation {
                true
            } else {
                false
            }
        }
    }
}
