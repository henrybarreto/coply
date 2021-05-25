use crate::chunk_data::ChunkData;
use crate::{ChunkOpt, ChunkRef};
use std::cell::RefCell;
use std::rc::Rc;

/// This structure represents a chain of chunks, what is a ChunkData and a next ChunkOpt.
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
    /// Create a ChunkOpt from a Chunk
    pub fn set_option(chunk: Chunk) -> ChunkOpt {
        Option::Some(Chunk::set_reference(chunk))
    }
    /// Create a ChunkRef from a Chunk
    pub fn set_reference(chunk: Chunk) -> ChunkRef {
        Rc::new(RefCell::new(chunk))
    }
    /// Get a ChunkRef from a ChunkOpt
    pub fn get_reference(chunk_opt: ChunkOpt) -> ChunkRef {
        chunk_opt.clone().expect("Could not clone the chunk opt")
    }
    /// Get a CheckOpt and return a Chunk
    pub fn get_from_option(chunk_opt: ChunkOpt) -> Chunk {
        Chunk::get_from_reference(chunk_opt.clone().expect("Cloud not get chunk from option"))
    }
    /// Get a ChunkRef and return a Chunk owned
    pub fn get_from_reference(chunk_ref: ChunkRef) -> Chunk {
        chunk_ref
            .clone()
            .try_borrow_mut()
            .expect("Could not get the borrow mut from the chunk ref")
            .to_owned()
    }
}
