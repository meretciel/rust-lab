

trait Account<T> {
    fn balance(&self) -> T;
}

struct Chase<T> {
    value: T
}


impl Account<i32> for Chase<i32> {
    fn balance(&self) -> i32 {
        100
    }
}

// impl<T> Account<T> for Chase<T> where T: Copy + !i32 {
//     fn balance(&self) -> T {
//         self.value
//     }
// }

// impl<'a> SayHello for &'a Foo {
//     fn hello(&self) {
//         println!("Hello from TestFoo");
//     }
// }

fn main() {
    let a:Chase<i32> = Chase{value: 10};
    println!("{}", a.balance());
}
