use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;


pub trait Event {
    type EventType;
    fn timestamp(&self) -> f32;
    fn event_type(&self) -> Self::EventType;
    fn event_id(&self) -> String;
    fn destination(&self) -> String;
}

pub trait EventQueue<E: Event> {
    fn add(&mut self, event: E);
    fn pop(&mut self) -> Option<E>;
}

pub trait Actor {
    type Event;
    type Queue;

    fn handle_event(&mut self, event: Self::Event, event_queue: &mut Self::Queue);
    fn name(&self) -> String;
}
