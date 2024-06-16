use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::BorrowMut;

#[derive(Debug)]
struct MyStruct{
    value: i32
}

impl MyStruct {
    fn set_value(&mut self, v: i32) {
        self.value = v;
    }
}

fn main() {
    let r = Rc::new(RefCell::new(MyStruct{value: 10}));
    println!("Initial value: {:?}", r);
    (*r).borrow_mut().set_value(20);
    println!("Updated value: {:?}", r);
}