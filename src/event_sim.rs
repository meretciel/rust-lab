use std::borrow::Borrow;


pub trait Event {
    type TyEventType;
    fn timestamp(&self) -> f32;
    fn event_type(&self) -> Self::TyEventType;
    fn event_id(&self) -> String;
    fn destination(&self) -> String;
}

pub trait EventQueue<E: Event> {
    fn add(&mut self, event: E);
    fn pop(&mut self) -> Option<E>;

    fn is_empty(&self) -> bool;
}

pub trait Actor {
    type TyEvent;
    type TyQueue;

    fn handle_event(&mut self, event: Self::TyEvent, event_queue: &mut Self::TyQueue);
    fn name(&self) -> String;
}
