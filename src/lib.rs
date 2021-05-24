use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fs::{File, Metadata},
    io::Read,
    rc::Rc,
};

pub mod buffer;
pub mod chunk;
pub mod chunk_data;
pub mod iteration;
pub mod reader;
pub mod writer;

use chunk::Chunk;

pub const CHUNK_SIZE: u8 = 128;
pub const CHUNKS_BY_BUFFER: u32 = 4;

pub type ChunkDataType = Vec<u8>;
type ChunkRef = Rc<RefCell<Chunk>>;
pub type ChunkOpt = Option<ChunkRef>;
