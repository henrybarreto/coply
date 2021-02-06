use std::{fs::{File, metadata}, io::Read};

use coply::coply::{Buffer, CHUNK_SIZE, ChunkData};

#[test]
fn creating_buffer_from_data() {
    let mut buffer = Buffer::new();
    let c_1 = buffer.add_chunk(ChunkData::Data(Box::new([6; 128])));
    let c_2 = buffer.add_chunk(ChunkData::Data(Box::new([7; 128])));
    let c_3 = buffer.add_chunk(ChunkData::Data(Box::new([8; 128])));

    assert_eq!(
        *c_1.data.unwrap(),
        *Box::new([6; 128]));
    assert_eq!(
        *c_2.data.unwrap(),
        *Box::new([7; 128]));
    assert_eq!(
        *c_3.data.unwrap(),
        *Box::new([8; 128]));
}


#[test]
fn creating_buffer_with_file_data() {
    let mut file = File::open("./test.txt").unwrap();
    let file_data = metadata("./test.txt").unwrap();
    let file_size = file_data.len();
    let times = {
        if (file_size / CHUNK_SIZE as u64) < 1 {
            1
        } else {
            (file_size / CHUNK_SIZE as u64) + 1
        }
    };

    let mut buffer = Box::new(Buffer::new());

    for _t in 0..times {
        let mut data = Box::new([0; CHUNK_SIZE as usize]);
        file.read(&mut data[..]).unwrap();
        buffer.add_chunk(ChunkData::Data(data.clone()));
    }

    println!("{:?}", buffer.chunks.to_owned());
}