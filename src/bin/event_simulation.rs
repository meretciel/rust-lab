use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::rc::Rc;
use csv::ErrorKind::Serialize;
use uuid::Uuid;


#[derive(Debug)]
enum EventType {
    Tick,
    Request,
    RequestComplete,
}


#[derive(Debug)]
struct Event {
    timestamp: f32,
    event_type: EventType,
    event_id: String,
    destination: String,
}

impl Event {
    fn new(timestamp: f32, event_type: EventType, destination: String) -> Event {
        Event {
            timestamp,
            event_type,
            event_id: Uuid::new_v4().to_string(),
            destination
        }
    }
}



trait EventQueue {
    type TypeEvent;
    fn add(&self, event: Self::TypeEvent);
    fn pop(&self) -> Option<Self::TypeEvent>;
}




trait Actor {
    type TypeQueue;

    fn handle_event(&mut self, event: Event, event_queue: Self::TypeQueue);
}

enum ServerState {
    Idle,
    Busy,
    // Failed,
    // Off,
}


struct RequestGenerator<Q> {
    address_id: String,
    timestamp: f32,
}

impl<Q> RequestGenerator<Q> {
    fn new(address_id: String, init_timestamp: f32) -> RequestGenerator<Q> {
        RequestGenerator {
            address_id,
            timestamp: init_timestamp,
        }
    }
}

impl<Q> Actor for RequestGenerator<Q>
where Q: EventQueue {
    type TypeQueue = Q;

    fn handle_event(&mut self, event: Event, event_queue: Self::TypeQueue) {
        self.timestamp = event.timestamp;
        event_queue.add(
            Event::new(self.timestamp + 1.0, EventType::Request, self.address_id.clone())
        );
    }
}


struct Server<Q> {
    address_id: String,
    state: ServerState,
    queue: VecDeque<Event>,
    timestamp: f32,
    next_idle_time: f32
}


impl<Q> Server<Q> {
    fn new(address_id: String, init_timestamp: f32) -> Server<Q> {
        Server {
            address_id,
            state: ServerState::Idle,
            queue: VecDeque::<Event>::new(),
            timestamp: init_timestamp,
            next_idle_time: init_timestamp,
        }
    }
}

impl<Q> Actor for Server<Q>
where Q: EventQueue {
    type TypeQueue = Q;

    fn handle_event(&mut self, event: Event, event_queue: Self::TypeQueue) {
        self.timestamp = event.timestamp;

        if let ServerState::Busy = self.state {
            if self.timestamp >= self.next_idle_time {
                self.state = ServerState::Idle;
                self.next_idle_time = self.timestamp;
            }
        }

        match self.state {
            ServerState::Idle => {
                if let EventType::Request = event.event_type {
                    let completion_time = self.timestamp + 10.;
                    let next_event = Event::new(
                        completion_time,
                        EventType::RequestComplete,
                        "".to_owned()
                    );
                    event_queue.add(next_event);
                    self.state = ServerState::Busy;
                    self.next_idle_time = completion_time;
                }

            }
            ServerState::Busy => {
                self.queue.push_back(event);
            }
        }
    }
}







fn main() -> Result<(), Box<dyn Error>> {
    let start_sim_timestamp = 0.0;
    let end_sim_timestamp = 200.;
    let event_queue = SimpleEventQueue::new();


    Ok(())
}