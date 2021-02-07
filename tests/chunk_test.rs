use coply::coply::*;
#[test]
fn creating_chunk_chain() {
    let mut c_1 = Chunk::new(ChunkData::Data(vec![6; 128]));
    let mut c_2 = Chunk::new(ChunkData::Data(vec![7; 128]));
    let mut c_3 = Chunk::new(ChunkData::Data(vec![8; 110]));

    c_1.next = ChunkOpt::None;
    c_2.next = Chunk::set_option(c_1);
    c_3.next = Chunk::set_option(c_2);
}