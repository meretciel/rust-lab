use uuid::Uuid;
use std::error::Error;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::mpsc::{channel, Sender};

use std::collections::VecDeque;
use std::{thread, time};

#[derive(Debug)]
enum Message {
    HealthCheck,
    Echo(i32),
}

#[derive(Debug)]
struct MessageWrapper {
    message_id: String,
    data: Message,
}

impl MessageWrapper {
    fn new(msg: Message) -> MessageWrapper {
        let msg_id = Uuid::new_v4().to_string();
        MessageWrapper {
            message_id : msg_id,
            data: msg
        }
    }
}

#[derive(Debug)]
enum ResponseMessage {
    StringData(String)
}

#[derive(Debug)]
struct Response {
    message_id: String,
    request_id: String,
    data: ResponseMessage,
}


fn main() -> Result<(), Box<dyn Error>> {
    let message_queue = Arc::new(Mutex::new(VecDeque::<MessageWrapper>::new()));
    let (sender, receiver) = channel();
    let mut thread_handles = Vec::new();

    for i in 0..5 {
        let sender_cloned: Sender<Response> = sender.clone();
        let queue = Arc::clone(&message_queue);

        let t = thread::spawn(move || {
            let mut fund = 5.0;
            while fund > 0. {
                let top = {
                    let mut guard = queue.lock().unwrap();
                    guard.pop_front()
                };

                if let Some(message_wrapper) = top {
                    fund -= 1.;
                    let message_id = message_wrapper.message_id;
                    let message_data = message_wrapper.data;
                    match message_data {
                        Message::HealthCheck => {}
                        Message::Echo(num) => {}
                    }
                } else {
                    fund -= 0.1;
                }
                thread::sleep(time::Duration::from_secs(3));
            }
        });
        thread_handles.push(t);
    }

    let mut guard = message_queue.lock().unwrap();
    guard.push_back(
        MessageWrapper::new(Message::HealthCheck)
    );
    drop(guard);

    while let Ok(response) = receiver.recv() {
        println!("Response: {:?}", response)
    }

    for item in thread_handles.into_iter() {
        item.join();
    }

    Ok(())
}