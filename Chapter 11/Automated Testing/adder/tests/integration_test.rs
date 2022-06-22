extern crate adder;


//using module that may be required for test setup
mod common;


#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4,adder::add_two(2));
}
