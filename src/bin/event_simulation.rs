
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::error::Error;
use uuid::Uuid;
use rust_lab::event_sim::{Event, Actor, EventQueue};

#[derive(Debug, Copy, Clone)]
enum EventType {
    Tick,
    Request,
    RequestComplete,
}


#[derive(Debug)]
struct EventData {
    timestamp: f32,
    event_type: EventType,
    event_id: String,
    destination: String,
}

impl PartialEq for EventData {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp.eq(&other.timestamp)
    }
}

impl Eq for EventData {}

impl PartialOrd for EventData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventData {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = Reverse(self.timestamp).partial_cmp(&Reverse(other.timestamp));

        if let Some(order) = result {
            if let Ordering::Equal = order {
                let this_char = self.event_id.chars().next().unwrap();
                let other_char = other.event_id.chars().next().unwrap();
                return this_char.cmp(&other_char);
            } else {
                return order;
            }
        } else {
            panic!("Timestamp cannot be NaN.");
        }
    }
}


impl EventData {
    fn new(timestamp: f32, event_type: EventType, destination: String) -> EventData {
        EventData {
            timestamp,
            event_type,
            event_id: Uuid::new_v4().to_string(),
            destination
        }
    }
}

impl Event for EventData {
    type TyEventType = EventType;

    fn timestamp(&self) -> f32 {
        self.timestamp
    }

    fn event_type(&self) -> Self::TyEventType {
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

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}


struct RequestGenerator {
    address_id: String,
    timestamp: f32,
    max_timestamp: f32
}

impl RequestGenerator {
    fn new(address_id: String, init_timestamp: f32, max_timestamp: f32) -> RequestGenerator {
        RequestGenerator {
            address_id,
            timestamp: init_timestamp,
            max_timestamp
        }
    }
}

impl Actor for RequestGenerator {
    type TyEvent = EventData;
    type TyQueue = SimpleQueue<EventData>;

    fn name(&self) -> String {
        self.address_id.clone()
    }

    fn handle_event(&mut self, event: Self::TyEvent, event_queue: &mut Self::TyQueue) {
        self.timestamp = event.timestamp;
        let next_timestamp = self.timestamp + 1.0;
        
        if next_timestamp <= self.max_timestamp {
            event_queue.add(
                EventData::new(next_timestamp, EventType::Tick, self.address_id.clone())
            );
        }

        event_queue.add(
            EventData::new(self.timestamp, EventType::Request, "Server_1".to_owned())
        );
    }
}


struct Server {
    address_id: String,
    state: ServerState,
    queue: VecDeque<EventData>,
    timestamp: f32,
}


impl Server {
    fn new(address_id: String, init_timestamp: f32) -> Server {
        Server {
            address_id,
            state: ServerState::Idle,
            queue: VecDeque::<EventData>::new(),
            timestamp: init_timestamp,
            
        }
    }
}


impl Actor for Server {
    type TyEvent = EventData;
    type TyQueue = SimpleQueue<EventData>;

    fn name(&self) -> String {
        self.address_id.clone()
    }

    fn handle_event(&mut self, event: Self::TyEvent, event_queue: &mut Self::TyQueue) {
        self.timestamp = event.timestamp;


        match self.state {
            ServerState::Idle => {
                if let EventType::Request = event.event_type {
                    let completion_time = self.timestamp + 10.;
                    let next_event = EventData::new(
                        completion_time,
                        EventType::RequestComplete,
                        self.address_id.clone()
                    );
                    event_queue.add(next_event);
                    self.state = ServerState::Busy;
                }

            }
            ServerState::Busy => {
                if let EventType::RequestComplete = event.event_type {
                    if let Some(queued_event) = self.queue.pop_front() {
                        let completion_time = self.timestamp + 10.;
                        let next_event = EventData::new(
                            completion_time,
                            EventType::RequestComplete,
                            self.address_id.clone()
                        );
                        event_queue.add(next_event);
                        self.state = ServerState::Busy;
                    } else {
                        self.state = ServerState::Idle;   
                    }
                } else {
                    self.queue.push_back(event);
                }
            }
        }
    }
}







fn main() -> Result<(), Box<dyn Error>> {
    let start_sim_timestamp = 0.0;
    let end_sim_timestamp = 60.;
    let mut event_queue = SimpleQueue::<EventData>::new();
    let actors: Vec<Box<dyn Actor<TyEvent=EventData, TyQueue=SimpleQueue<EventData>>>> = vec![
      Box::new(RequestGenerator::new("RequestGenerator".to_owned(), start_sim_timestamp, end_sim_timestamp / 2.)),
      Box::new(Server::new("Server_1".to_owned(), start_sim_timestamp)),
    ];

    let mut name_to_actors = HashMap::new();
    for item in actors.into_iter() {
        name_to_actors.insert(item.name(), item);
    }

    // Add seed messages.
    event_queue.add(EventData::new(
      start_sim_timestamp,
      EventType::Tick,
      "RequestGenerator".to_owned()
    ));

    let mut count = 0;

    while !event_queue.is_empty() {
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