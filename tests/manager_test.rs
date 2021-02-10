use std::{fs::File, io::Write};

use coply::coply::Manager;

#[test]
fn creating_a_clone_file() {
    let mut manager = Manager::new();
    let buffers = manager.read("test.txt");
    let mut file = File::create("clone.txt").expect("Could not create the clone file");
    for buffer in buffers {
        file.write(&buffer.join_data()).unwrap();
    }
}
