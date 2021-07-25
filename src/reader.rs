use crate::buffer::Buffer;
use crate::chunk_data::ChunkData;
use crate::iteration::Iteration;
use crate::CHUNK_SIZE;
use std::borrow::BorrowMut;
use std::fs::{File, Metadata};
use std::io::Read;

/// This structure is used to execute reading operation in a file
#[derive(Debug)]
pub struct Reader {
    // pub buffers: Vec<Buffer>,
    pub file: File,
    pub file_info: Metadata,
    pub iteration: Iteration,
}

impl Reader {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let file_info = file.metadata().unwrap();
        let file_size = file_info.len();
        let iteration = Iteration::new(file_size, CHUNK_SIZE);
        Reader {
            // buffers: Vec::new(),
            file,
            file_info,
            iteration,
        }
    }
    /// Read the file into a vector of Buffers
    pub fn read(&mut self) -> Buffer {
        // pub fn read(&mut self) -> Vec<Buffer> {
        // What is happening here?
        if self.iteration.actual_step <= self.iteration.steps {
            // Iteration and Reader are hardly linked...
            let file = self.file.borrow_mut();
            let mut buffer = Buffer::new();
            let iteration = self.iteration.borrow_mut();
            iteration.iter(|bytes| {
                let mut data: Vec<u8> = vec![0; bytes as usize];
                file.borrow_mut().read(&mut data).unwrap();
                buffer.borrow_mut().add_data(ChunkData::Data(data));
            });

            // let mut buffers = self.buffers.to_owned();
            // buffers.push(buffer);
            // self.buffers = buffers.clone();
            buffer
        } else {
            /*
             * iteration.iter and its internal condition checks bytes size to read
             * in this scope, the condition avoid panic of the line 48
             * I guess it is highly coupled
             * REFACTOR EITHER READER, ITERATION OR BOTH
             */
            // self.buffers.clone().to_owned()
            Buffer::new()
        }
    }
}
