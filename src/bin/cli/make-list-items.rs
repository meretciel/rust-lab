
use std::error::Error;
use std::io;


fn main() -> Result<(), Box<dyn Error>> {
    let lines = io::stdin().lines();
    let mut outputs = Vec::new();

    for line in lines {
        let l = line?;
        let v = l.trim();
        if !v.is_empty() {
            outputs.push(v.to_string());
        }
    }

    println!("\n\n{}\n\n", "-".repeat(60));
    for line in &outputs {
        println!("\"{}\",", line.trim());
    }

    println!("\n\n{}\n\n", "-".repeat(60));
    Ok(())
}
