use std::borrow::Borrow;


pub trait Event {
    type TyEventType;
    fn timestamp(&self) -> f32;
    fn event_type(&self) -> Self::TyEventType;
    fn event_id(&self) -> String;
    fn destination(&self) -> String;
}

pub trait EventQueue {
    type TyEvent: Event;

    fn add(&mut self, event: Self::TyEvent);
    fn pop(&mut self) -> Option<Self::TyEvent>;

    fn is_empty(&self) -> bool;
}

pub trait Actor {
    type TyEvent;
    type TyQueue;

    fn handle_event(&mut self, event: Self::TyEvent, event_queue: &mut Self::TyQueue);
    fn name(&self) -> String;
}
