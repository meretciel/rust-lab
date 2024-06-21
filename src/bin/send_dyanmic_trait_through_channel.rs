use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, mpsc, Mutex};
use std::{thread, time};
use std::any::Any;

// Resources:
// https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object
// https://doc.rust-lang.org/std/any/index.html



type MsgSender = Sender<Box<dyn Message + Send>>;

trait Message {
    fn get_message_id(&self) -> i64;
    fn get_sender(&self) -> Option<MsgSender>;

    fn as_any(&self) -> &dyn Any;
}


struct HelloMessage {
    message_id: i64,
    sender: Option<MsgSender>
}

struct GoodbyeMessage {
    message_id: i64,
    sender: Option<MsgSender>
}


impl Message for HelloMessage {
    fn get_message_id(&self) -> i64 {
        self.message_id
    }

    fn get_sender(&self) -> Option<MsgSender> {
        self.sender.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Message for GoodbyeMessage {
    fn get_message_id(&self) -> i64 {
        self.message_id
    }

    fn get_sender(&self) -> Option<MsgSender> {
        self.sender.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}



fn main() {
    let (sender_toby, receiver_toby): (MsgSender, Receiver<Box<dyn Message + Send>>) = mpsc::channel();
    let address_toby = sender_toby.clone();
    let handler_toby = thread::spawn(move || {
        let mut message_id = 100;
        while let Ok(msg) = receiver_toby.recv() {
            if let Some(m) = msg.as_any().downcast_ref::<HelloMessage>() {
                println!("[Toby] Received an HelloMessage: {}", m.get_message_id());
                if let Some(sender) = m.get_sender() {
                    message_id += 1;
                    thread::sleep(time::Duration::from_secs(1));
                    if message_id < 110 {
                        sender.send(Box::new(HelloMessage{message_id: message_id, sender: Some(address_toby.clone())})).unwrap();
                    } else {
                        sender.send(Box::new(GoodbyeMessage{message_id: message_id, sender: Some(address_toby.clone())})).unwrap();
                    }
                }
            }

            if let Some(m) = msg.as_any().downcast_ref::<GoodbyeMessage>() {
                println!("[Toby] Received an GoodbyeMessage: {}", m.get_message_id());
            }
        }
    });

    let (sender_alice, receiver_alice): (MsgSender, Receiver<Box<dyn Message + Send>>) = mpsc::channel();
    let address_alice = sender_alice.clone();
    let handler_alice = thread::spawn(move || {
        let mut message_id = 600;
        while let Ok(msg) = receiver_alice.recv() {
            if let Some(m) = msg.as_any().downcast_ref::<HelloMessage>() {
                println!("[Alice] Received an HelloMessage: {}", m.get_message_id());
                if let Some(sender) = m.get_sender() {
                    message_id += 1;
                    thread::sleep(time::Duration::from_secs(1));
                    sender.send(Box::new(HelloMessage{message_id: message_id, sender: Some(address_alice.clone())})).unwrap();
                }
            }

            if let Some(m) = msg.as_any().downcast_ref::<GoodbyeMessage>() {
                println!("[Alice] Received an GoodbyeMessage: {}", m.get_message_id());
            }
        }
    });


    println!("Wait 3 seconds.");
    thread::sleep(time::Duration::from_secs(3));

    let start_msg = Box::new(HelloMessage{message_id: 10, sender: Some(sender_alice.clone())});
    sender_toby.send(start_msg).unwrap();

    handler_toby.join().unwrap();
    handler_alice.join().unwrap();
}