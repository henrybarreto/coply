use crate::buffer::Buffer;
use std::fs::File;
use std::io::Write;

/// This structure is used to execute writing operation in a file
#[derive(Debug)]
pub struct Writer {
    pub file: File,
}
impl Writer {
    pub fn new(file_path: &str) -> Self {
        let file = File::create(file_path).expect("Could not open the file from writer");
        Writer { file }
    }
    /// Writes in a file the Buffer data
    pub fn write(&mut self, buffer: Buffer) {
        self.file
            .write(&mut buffer.join_data())
            .expect("Could not write the buffer to a archive");
    }
}
