use coply::coply::{Buffer, ChunkData};

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

/*fn file() {
    let mut file = File::open("test.txt").unwrap();
    let file_info = metadata("test.txt").unwrap();
    let file_size = file_info.len();
    let int = (file_size / CHUNK_SIZE as u64) as u8;
    let remain = (file_size % CHUNK_SIZE as u64) as u8;
    let mut buffer = Buffer::new();

    for _t in 0..int+1 {
        if ! _t > 4 {
            let mut data: Vec<u8> = vec![0; remain as usize];
            file.read(&mut data[..]).unwrap();
            buffer.add_data(ChunkData::Data(data));
        } else {
            let mut data: Vec<u8> = vec![0; CHUNK_SIZE as usize];
            file.read(&mut data[..]).unwrap();
            buffer.add_data(ChunkData::Data(data));
        }
    }
    let mut file_2 = File::create("test_2.txt").unwrap();
    file_2.write(&buffer.join_data()).unwrap();
}*/
