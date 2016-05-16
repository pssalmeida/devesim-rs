extern crate rand;

use std::collections::binary_heap::BinaryHeap;
use std::cmp::Ordering;

pub mod distributions;

pub trait Event {
    fn time(&self) -> u64;
}

struct QE<E: Event>(E);

impl<E: Event> Eq for QE<E> { }

impl<E: Event> Event for QE<E> {
    fn time(&self) -> u64 { self.0.time() }
}

impl<E: Event> PartialEq for QE<E> {
    fn eq(&self, other: &Self) -> bool {
        self.time() == other.time()
    }
}

impl<E: Event> Ord for QE<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time().cmp(&self.time())
    }
}

impl<E: Event> PartialOrd for QE<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Simulator<E: Event> {
    time: u64,
    events : BinaryHeap<QE<E>>,
}


impl<E: Event> Simulator<E> {

    pub fn new() -> Simulator<E> {
        Simulator {
            time: 0,
            events: BinaryHeap::new(),
        }
    }

    pub fn push_event(&mut self, event: E) {
        assert!(self.time <= event.time());
        self.events.push(QE(event));
    }

    pub fn pop_event(&mut self) -> Option<E> {
        match self.events.pop() {
            None => None,
            Some(QE(event)) => {
                self.time = event.time();
                Some(event)
            }
        }
    }

    pub fn time(&self) -> u64 {
        self.time
    }

}

#[cfg(test)]
mod tests {
}

