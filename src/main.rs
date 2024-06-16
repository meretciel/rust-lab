
mod level1 {
    pub fn foo() {println!("foo")}
}

#[derive(Debug)]
struct Elem {
    value: i32
}

fn foo(x: Elem) {
    println!("foo(x:i32) is called. {:?}", x)
}

fn boo(x: &Elem) {
    println!("boo(x:i32) is called. {:?}", x)
}

fn koo(&x: &Elem) {
    println!("koo(x:i32) is called. {:?}", x)
}

fn main() {
    let a = Elem {value: 10};
    let b = a;
    // foo(a);
  
}