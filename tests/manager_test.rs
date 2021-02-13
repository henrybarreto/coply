use std::{fs::File, io::Write};

use coply::coply::Manager;

#[test]
fn creating_a_clone_file() {
    let mut manager = Manager::new("LICENSE");
    let buffers_1 = manager.read();
    let buffers_2 = manager.read();
    let buffers_3 = manager.read();
    let buffers_4 = manager.read();
    let buffers_5 = manager.read();
    //println!("{:?}", buffers_1);
    //println!("\n\n{:?}", buffers_2);
    //println!("\n\n{:?}", buffers_3);
    //println!("\n\n{:?}", buffers_4);
    //println!("\n\n{:?}", buffers_5);
    let mut file = File::create("clone_test.txt").expect("Could not create the clone file");
    for buffer in buffers_5 {
        file.write(&buffer.join_data()).unwrap();
    }
}
