use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:9236").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let mut buf_reader = BufReader::new(&stream);
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        let mut buffer = String::new();
        loop {
            if let Ok(n) = buf_reader.read_line(&mut buffer) {
                let msg = &buffer[..(buffer.len()-1)];
                println!("received a message: {msg}");
                buffer.clear();
            }
        }
    }

    Ok(())
}
