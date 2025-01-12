

fn foo() { println!("foo"); }
fn bar() { println!("bar"); }


fn main() {
    let f = &foo;
    f();
}