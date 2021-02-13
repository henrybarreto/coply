
use std::{fs::File, io::Write};

use coply::coply::{Reader};

#[test]
fn reading_test_file() {
    let mut reader = Reader::new("test.txt");
    let _buffers_1 = reader.read(); // 0+4
    let _buffers_2 = reader.read(); // 4+4
    let _buffers_3 = reader.read(); //8 + 4 
    let _buffers_4 = reader.read(); //12 +4
    let buffers_5 = reader.read();
    
    let mut file = File::create("clone.txt").unwrap();
    for buffer in buffers_5 {
        file.write(&mut buffer.join_data()).unwrap();
    }
}
