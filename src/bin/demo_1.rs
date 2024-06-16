#[allow(warnings)]
fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = (0..10).map(|x| {-x}).collect();
    println!("Example of map: {:?}", v);
    let v: Vec<i32> = (0..10).filter_map(|x| {if x % 2 == 0 {Some(x)} else {None}}).collect();
    println!("Example of filter_map: {:?}", v);

    let data = vec![
      "",
      "#",
      "@@",
      "???"
    ];
    let s: Vec<char> = (1..=3).flat_map(|x| {
        (&data[x]).chars()
    }).collect();
    println!("Example of flat_map: {:?}", s);

    let data= vec![vec!["#"], vec!["@", "@"], vec!["?", "?", "?"]];
    let s: Vec<_> = data.iter().flatten().collect();
    println!("Example of flatten: {:?}", s);

    let v: Vec<_> = (0..10).take(5).collect();
    println!("Example of take: {:?}", v);

    let v: Vec<_> = (0..10).take_while(|x| *x < 5).collect();
    println!("Example of take_while: {:?}", v);

    let v: Vec<_> = (0..10).skip(5).collect();
    println!("Example of skip: {:?}", v);

    let v: Vec<_> = (0..10).skip_while(|x| *x < 5).collect();
    println!("Example of skip_while: {:?}", v);

    let v:Vec<_> = (0..10).rev().collect();
    println!("Example of rev: {:?}", v);

    let v: Vec<_> = (0..10).chain(vec![-1, -2, -3]).collect();
    println!("Example of chain: {:?}", v);

    let v: Vec<_> = (0..10).rev().enumerate().collect();
    println!("Example of enumerate: {:?}", v);

    let v: Vec<_> = (0..10).zip(vec![-1, -2, -3]).collect();
    println!("Example of zip: {:?}", v);

    // --- by_ref ---
    // The purpose of by_ref is to re-use the iterator.
    let mut v = (0..10).into_iter();
    println!("Example of by_ref:");
    let s1: Vec<_> = v.by_ref().map(|x| x + 1).collect();
    println!("\t s1: {:?}", s1);
    let s2: Vec<_> = v.map(|x| x - 1).collect();
    println!("\t s2: {:?}", s1);

    let v:Vec<_>  = (0..3).cycle().take(10).collect();
    println!("Example of cycle: {:?}", v);

    let (odd, even): (Vec<_>, Vec<_>) = (0..10).partition(|x| x % 2 == 1);
    println!("Example of partition:");
    println!("\tOdd numbers: {:?}", odd);
    println!("\tEven numbers; {:?}", even);
}
