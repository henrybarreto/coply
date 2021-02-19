pub mod coply {
    use std::{borrow::{BorrowMut}, cell::RefCell, fs::{File, Metadata}, io::{Read}, rc::Rc};
    use std::io::Write;

    pub const CHUNK_SIZE: u8 = 128;
    pub const CHUNKS_BY_BUFFER: u32 = 4;

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
            chunk_opt.clone()
                .expect("Could not clone the chunk opt")
        }
        pub fn get_from_option(chunk_opt: ChunkOpt) -> Chunk {
            Chunk::get_from_reference(chunk_opt.clone()
                .expect("Cloud not get chunk from option"))
        }
        pub fn get_from_reference(chunk_ref: ChunkRef) -> Chunk {
            chunk_ref
                .clone()
                .try_borrow_mut()
                .expect("Could not get the borrow mut from the chunk ref")
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
    #[derive(Debug)]
    pub struct Writer {
        pub file: File
    }
    impl Writer {
        pub fn new(file_path: &str) -> Self {
            let file = File::create(file_path)
                .expect("Could not open the file from writer");
            Writer {
                file
            }
        }
        pub fn write(&mut self, buffer: Buffer) {
            self.file.write(&mut buffer.join_data())
                .expect("Could not write the buffer to a archive");
        }
    }

    #[derive(Debug)]
    pub struct Reader {
        pub buffers: Vec<Buffer>,
        pub file: File,
        pub file_info: Metadata,
        pub interation: Iteration
    }

    impl Reader {
        pub fn new(file_path: &str) -> Self {
            let file = File::open(file_path).unwrap();
            let file_info = file.metadata().unwrap();
            let file_size = file_info.len();
            let interation = Iteration::new(file_size, CHUNK_SIZE);
            Reader {
                buffers: Vec::new(),
                file,
                file_info,
                interation
            }
        }
        pub fn read(&mut self) -> Vec<Buffer> { // What is happening here?
            if self.interation.actual_step <= self.interation.steps { // Iteration and Reader are hardly linked...
                let file = self.file.borrow_mut();
                let mut buffer = Buffer::new();
                let interation = self.interation.borrow_mut();
                interation.iter(|bytes| {
                    let mut data: Vec<u8> = vec![0; bytes as usize];
                    file.borrow_mut().read(&mut data).unwrap();
                    buffer.borrow_mut().add_data(ChunkData::Data(data));
                });

                let mut buffers = self.buffers.to_owned();
                buffers.push(buffer);
                self.buffers = buffers.clone();
                buffers
            } else {
                /*
                 * iteration.iter and its internal condition checks bytes size to read
                 * in this scope, the condition avoid panic of the line 48
                 * I guess it is highly coupled
                 * REFACTOR EITHER READER, ITERATION OR BOTH
                 */
                self.buffers.to_owned()
            }
        }
    }
    

    #[derive(Debug, Clone)]
    pub struct Iteration {
        pub steps: u32,
        pub actual_step: u32,
        pub bytes: u8,
        pub last_bytes: u8,
    }
    impl Iteration {
        pub fn new(file_size: u64, chunk_size: u8) -> Self {
            if (file_size % CHUNK_SIZE as u64) == 0 {
                Iteration {
                    steps: (file_size / chunk_size as u64) as u32,
                    actual_step: 1,
                    bytes: chunk_size,
                    last_bytes: chunk_size,
                }
            } else {
                Iteration {
                    steps: ((file_size / chunk_size as u64) + 1) as u32,
                    actual_step: 1,
                    bytes: chunk_size,
                    last_bytes: (file_size % chunk_size as u64) as u8,
                }
            }
        }
        pub fn iter<N>(&mut self, mut iter: N)
        where
            N: FnMut(u8) -> (),
        {
            for _i in 0..CHUNKS_BY_BUFFER {
                if self.actual_step <= self.steps {
                    if self.is_last(self.actual_step) {
                        self.actual_step = self.actual_step + 1;
                        iter(self.last_bytes);
                    } else {
                        self.actual_step = self.actual_step + 1;
                        iter(self.bytes);
                    }
                } else {
                    break;
                }
            }
        }
        pub fn is_last(&self, iteration: u32) -> bool {
            if self.steps == iteration {
                true
            } else {
                false
            }
        }
    }
}
