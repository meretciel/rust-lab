use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct MyStruct{
    value: i32
}

struct MyStruct2 {
    borrowed: MyStruct
}

impl std::borrow::Borrow<MyStruct> for MyStruct2 {
    fn borrow(&self) -> &MyStruct {
        &self.borrowed
    }
}

impl std::borrow::BorrowMut<MyStruct> for MyStruct2 {
    fn borrow_mut(&mut self) -> &mut MyStruct {
        &mut (self.borrowed)
    }
}

impl MyStruct {
    fn set_value(&mut self, v: i32) {
        self.value = v;
    }
}

fn main() {
    let r = Rc::new(RefCell::new(MyStruct{value: 10}));
    println!("Initial value: {:?}", r);
    r.borrow_mut().set_value(20);
    println!("Updated value: {:?}", r);
}