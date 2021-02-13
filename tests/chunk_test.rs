use coply::coply::*;
#[test]
fn creating_chunk_chain() {
    let mut c_1 = Chunk::new(ChunkData::Data(vec![6; CHUNK_SIZE as usize]));
    let mut c_2 = Chunk::new(ChunkData::Data(vec![7; CHUNK_SIZE as usize]));
    let mut c_3 = Chunk::new(ChunkData::Data(vec![8; 110]));

    c_1.next = ChunkOpt::None;
    c_2.next = Chunk::set_option(c_1.clone());
    c_3.next = Chunk::set_option(c_2.clone());
    if let ChunkData::Data(data) = c_3.clone().data {
        assert_eq!(data, vec![8; 110]);
    }
    if let ChunkData::Data(data) = c_2.clone().data {
        assert_eq!(data, vec![7; CHUNK_SIZE as usize]);
    }
    if let ChunkData::Data(data) = c_1.clone().data {
        assert_eq!(data, vec![6; CHUNK_SIZE as usize]);
    }
}
