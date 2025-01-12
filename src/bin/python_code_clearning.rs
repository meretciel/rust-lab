use std::io;
use std::io::Write;
use std::{thread, time};

fn main() {

    // while let io::Result::Ok(n) = io::stdin().read_line(&mut buffer) {
    //     // io::stdout().write(buffer[3..].as_bytes()).unwrap();
    //     println!("{n}");
    //     thread::sleep(time::Duration::from_secs(1));
    // }
    println!("{}", "-".repeat(80));
    let mut is_func_def = false;
    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            io::Result::Ok(n) if n > 0 => {
                if n >= 3 {
                    if buffer.starts_with("...") {
                        is_func_def = true;
                    } else if is_func_def {
                        is_func_def = false;
                        println!("    {}", buffer);
                    } else {
                        is_func_def = false;
                        println!("{}", &buffer[3..].trim());
                    }
                } else {
                    println!("{}", buffer);
                }
            }
            _ => {break;}
        }
    }
    println!("{}", "-".repeat(80));
}