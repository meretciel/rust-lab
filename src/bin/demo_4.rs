
trait SayHello {
    fn hello(&self);
}

struct Foo(i32);

impl SayHello for Foo {
    fn hello(&self) {
        println!("hello from Foo");
    }
}

// impl<'a> SayHello for &'a Foo {
//     fn hello(&self) {
//         println!("Hello from TestFoo");
//     }
// }

fn main() {
    // let x = Foo(10);
    // let y = &x;
    // SayHello::hello(y);
    let v = vec![1,2,3];
    for item in (&v).into_iter() {
        println!("{item}");
    }
    println!("{}", v.len());
}
