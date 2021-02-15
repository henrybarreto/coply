use coply::coply::{Reader, Writer};

#[test]
fn reading_and_writing_test_file() {
    let mut reader = Reader::new("test.txt");
    let mut writer = Writer::new("test_clone.txt");
    let _buffers_1 = reader.read(); // 0+4
    let _buffers_2 = reader.read(); // 4+4
    let _buffers_3 = reader.read(); //8 + 4 
    let _buffers_4 = reader.read(); //12 +4
    let buffers_5 = reader.read();
    
    for buffer in buffers_5 {
        writer.write(buffer);
    }
}
