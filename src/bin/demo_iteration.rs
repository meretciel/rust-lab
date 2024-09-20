#[derive(Debug)]
struct Record {
    x: i32
}

fn main() {
    let v = vec![
        Record { x: 10 },
        Record { x: 20 },
    ];
    //
    //     for &item in v.iter() {
    //         println!("{:?}", item);
    //     }
    //
    //     println!("---");
    //
    //     for item in v.iter() {
    //         println!("{:?}", item);
    //     }
    // }
}