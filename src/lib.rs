//! THIS IS A STUDY PROJECT, EVERYTHING HERE CAN AND WILL BE IMPROVED AS SOON AS POSSIBLE.
//! This is a library crate what prove structures to building a copy file tool.

use std::{cell::RefCell, rc::Rc};

pub mod buffer;
pub mod chunk;
pub mod chunk_data;
pub mod iteration;
pub mod reader;
pub mod writer;

use chunk::Chunk;

/// Default size of each chunk
pub const CHUNK_SIZE: u8 = 128;
/// Default quantity of chunk by buffer
pub const CHUNKS_BY_BUFFER: u32 = 4;

/// Type of each Chunk
pub type ChunkDataType = Vec<u8>;
/// Reference Type of Chunk
pub type ChunkRef = Rc<RefCell<Chunk>>;
/// This type can be either a ChunkRef or Empty value
pub type ChunkOpt = Option<ChunkRef>;
