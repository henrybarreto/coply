use coply::reader::Reader;
use coply::writer::Writer;

#[test]
fn reading_and_writing_test_file() {
    let mut reader = Reader::new("test.txt");
    let mut writer = Writer::new("test_clone.txt");
    // let _buffers_1 = reader.read(); // 0+4
    // let _buffers_2 = reader.read(); // 4+4
    // let _buffers_3 = reader.read(); //8 + 4
    // let _buffers_4 = reader.read(); //12 +4
    // let buffers_5 = reader.read();
    let steps = if reader.iteration.steps % 4 == 0 {
        reader.iteration.steps / 4
    } else {
        (reader.iteration.steps / 4) + 1
    };

    if steps <= 0 {
        for _i in 0..1 {
            let buffer = reader.read();
            writer.write(buffer);
        }
    } else {
        for _i in 0..steps {
            let buffer = reader.read();
            writer.write(buffer);
        }
    }
}
