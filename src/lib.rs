extern crate rand;

use std::collections::binary_heap::BinaryHeap;
use std::cmp::Ordering;

pub mod distributions;

struct TSE<E> {
    time: u64,
    data: E,
}

impl<E> Eq for TSE<E> { }

impl<E> PartialEq for TSE<E> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl<E> Ord for TSE<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl<E> PartialOrd for TSE<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Simulator<E> {
    time: u64,
    events : BinaryHeap<TSE<E>>,
}


impl<E> Simulator<E> {

    pub fn new() -> Simulator<E> {
        Simulator {
            time: 0,
            events: BinaryHeap::new(),
        }
    }

    pub fn push_event(&mut self, event: E, delta_time: u64) {
        let time = self.time + delta_time;
        self.events.push(TSE {time : time, data : event});
    }

    pub fn pop_event(&mut self) -> Option<E> {
        match self.events.pop() {
            None => None,
            Some(TSE { time, data }) => {
                self.time = time;
                Some(data)
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

