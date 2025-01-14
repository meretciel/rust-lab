use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::error::Error;
use std::rc::Rc;
use csv::ErrorKind::Serialize;
use uuid::fmt::Simple;
use uuid::Uuid;
use rust_lab::event_sim::{Event, Actor, EventQueue};


#[derive(Debug, Copy, Clone)]
enum EventType {
    Tick,
    Request,
    RequestComplete,
}


#[derive(Debug)]
struct SimpleEvent {
    timestamp: f32,
    event_type: EventType,
    event_id: String,
    destination: String,
}

impl PartialEq for SimpleEvent {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp.eq(&other.timestamp)
    }
}

impl Eq for SimpleEvent {}

impl PartialOrd for SimpleEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SimpleEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = Reverse(self.timestamp).partial_cmp(&Reverse(other.timestamp));

        if let Some(order) = result {
            return order;
        } else {
            panic!("Timestamp cannot be NaN.");
        }
    }
}


impl SimpleEvent {
    fn new(timestamp: f32, event_type: EventType, destination: String) -> Event {
        Event {
            timestamp,
            event_type,
            event_id: Uuid::new_v4().to_string(),
            destination
        }
    }
}

impl Event for SimpleEvent {
    type EventType = EventType;

    fn timestamp(&self) -> f32 {
        self.timestamp
    }

    fn event_type(&self) -> Self::EventType {
        self.event_type
    }

    fn event_id(&self) -> String {
        self.event_id.clone()
    }

    fn destination(&self) -> String {
        self.destination.clone()
    }
}


enum ServerState {
    Idle,
    Busy,
    // Failed,
    // Off,
}

struct SimpleQueue<E> {
    queue: BinaryHeap<E>
}

impl<E: Ord> SimpleQueue<E> {
    fn new() -> SimpleQueue<E> {
        SimpleQueue {
            queue: BinaryHeap::new()
        }
    }
}

impl<E> EventQueue<E> for SimpleQueue<E>
where E: Ord + Event {
    fn add(&mut self, event: E) {
        self.queue.push(event);
    }

    fn pop(&mut self) -> Option<E> {
        self.queue.pop()
    }
}


struct RequestGenerator {
    address_id: String,
    timestamp: f32,
}

impl RequestGenerator {
    fn new(address_id: String, init_timestamp: f32) -> RequestGenerator {
        RequestGenerator {
            address_id,
            timestamp: init_timestamp,
        }
    }
}

impl Actor for RequestGenerator {
    type Event = SimpleEvent;
    type Queue = SimpleQueue<SimpleEvent>;

    fn name(&self) -> String {
        self.address_id.clone()
    }

    fn handle_event(&mut self, event: Self::Event, event_queue: Self::Queue) {
        self.timestamp = event.timestamp;
        event_queue.add(
            SimpleEvent::new(self.timestamp + 1.0, EventType::Request, self.address_id.clone())
        );
    }
}


struct Server {
    address_id: String,
    state: ServerState,
    queue: VecDeque<SimpleEvent>,
    timestamp: f32,
    next_idle_time: f32
}
//
//
// impl<Q> Server<Q> {
//     fn new(address_id: String, init_timestamp: f32) -> Server<Q> {
//         Server {
//             address_id,
//             state: ServerState::Idle,
//             queue: VecDeque::<Event>::new(),
//             timestamp: init_timestamp,
//             next_idle_time: init_timestamp,
//         }
//     }
// }
//
// impl<Q> Actor for Server<Q>
// where Q: EventQueue {
//     type TypeQueue = Q;
//
//     fn handle_event(&mut self, event: Event, event_queue: Self::TypeQueue) {
//         self.timestamp = event.timestamp;
//
//         if let ServerState::Busy = self.state {
//             if self.timestamp >= self.next_idle_time {
//                 self.state = ServerState::Idle;
//                 self.next_idle_time = self.timestamp;
//             }
//         }
//
//         match self.state {
//             ServerState::Idle => {
//                 if let EventType::Request = event.event_type {
//                     let completion_time = self.timestamp + 10.;
//                     let next_event = Event::new(
//                         completion_time,
//                         EventType::RequestComplete,
//                         "".to_owned()
//                     );
//                     event_queue.add(next_event);
//                     self.state = ServerState::Busy;
//                     self.next_idle_time = completion_time;
//                 }
//
//             }
//             ServerState::Busy => {
//                 self.queue.push_back(event);
//             }
//         }
//     }
// }







fn main() -> Result<(), Box<dyn Error>> {
    let start_sim_timestamp = 0.0;
    let end_sim_timestamp = 200.;
    let mut event_queue = SimpleQueue::<SimpleEvent>::new();
    let actors: Vec<Box<dyn Actor<Event=SimpleEvent, Queue=SimpleQueue<SimpleEvent>>>> = vec![
      Box::new(RequestGenerator::new("RequestGenerator".to_owned(), start_sim_timestamp))
    ];

    let mut name_to_actors = HashMap::new();
    for item in actors.into_iter() {
        name_to_actors.insert(item.name(), item);
    }

    // Add seed messages.
    event_queue.add(SimpleEvent::new(
      start_sim_timestamp,
      EventType::Request,
      "RequestGenerator".to_owned()
    ));

    let mut count = 0;

    while count < 1000 {
        if let Some(event) = event_queue.pop() {
            println!("event: {:?}", event);
            let actor = name_to_actors.get_mut(&event.destination);
            if let Some(dest) = actor {
                dest.handle_event(event, &mut event_queue);
            }
        }
        count += 1;
    }

    Ok(())
}