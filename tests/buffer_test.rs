use coply::buffer::Buffer;
use coply::chunk_data::ChunkData;

#[test]
fn joining_all_data_from_buffer() {
    let mut buffer = Buffer::new();
    let _c_1 = buffer.add_data(ChunkData::Data(vec![6; 128]));
    let _c_2 = buffer.add_data(ChunkData::Data(vec![7; 128]));
    let _c_3 = buffer.add_data(ChunkData::Data(vec![8; 120]));

    assert_eq!(
        buffer.join_data().clone(),
        [vec![6; 128], vec![7; 128], vec![8; 120],].concat()
    );
}
