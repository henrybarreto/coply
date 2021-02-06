use std::{cell::RefCell, rc::Rc};

use coply::coply::*;

#[test]
fn check_chunk_creation() {
    let c_1 = Rc::new(RefCell::new(Chunk::new(ChunkData::Data(Box::new([1; 128])))));
    let c_2 = Rc::new(RefCell::new(Chunk::new(ChunkData::Data(Box::new([2; 128])))));
    let c_3 = Rc::new(RefCell::new(Chunk::new(ChunkData::Data(Box::new([3; 128])))));

    c_3.clone().try_borrow_mut().unwrap().next = None;
    c_2.clone().try_borrow_mut().unwrap().next = Some(c_3.clone());
    c_1.clone().try_borrow_mut().unwrap().next = Some(c_2.clone());

    assert_eq!(
        *c_3.clone().try_borrow_mut().unwrap().data.unwrap(),
        *Box::new([3; 128]));
    assert_eq!(
        *c_2.clone().try_borrow_mut().unwrap().data.unwrap(),
        *Box::new([2; 128]));
    assert_eq!(
        *c_1.clone().try_borrow_mut().unwrap().data.unwrap(),
        *Box::new([1; 128]));

}
#[test]
fn check_chunk_change_through_reference() {
    let c_1 = Rc::new(RefCell::new(Chunk::new(ChunkData::Data(Box::new([1; 128])))));
    let c_2 = Rc::new(RefCell::new(Chunk::new(ChunkData::Data(Box::new([2; 128])))));
    let c_3 = Rc::new(RefCell::new(Chunk::new(ChunkData::Data(Box::new([3; 128])))));

    c_3.clone().try_borrow_mut().unwrap().next = None;
    c_2.clone().try_borrow_mut().unwrap().next = Some(c_3.clone());
    c_1.clone().try_borrow_mut().unwrap().next = Some(c_2.clone());

    c_2.clone().try_borrow_mut().unwrap().data = ChunkData::Data(Box::new([5; 128]));

    assert_eq!(
        *c_2.clone().try_borrow_mut().unwrap().data.unwrap(),
        *Box::new([5; 128]));
}