extern crate rand;

use std::collections::binary_heap::BinaryHeap;
use std::cmp::Ordering;

pub mod distributions;

pub trait Event {
    fn time(&self) -> u64;
}

pub trait State<E: Event> {
    fn handle(&mut self, event: E) -> Vec<E>;
    fn statistics(&mut self, time: u64);
}

struct QE<E: Event>(E);

impl<E: Event> Eq for QE<E> { }

impl<E: Event> PartialEq for QE<E> {
    fn eq(&self, other: &Self) -> bool {
        self.0.time() == other.0.time()
    }
}

impl<E: Event> Ord for QE<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.time().cmp(&self.0.time())
    }
}

impl<E: Event> PartialOrd for QE<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Simulator<E: Event, S: State<E>> {
    time: u64,
    events : BinaryHeap<QE<E>>,
    state: S,
}


impl<E: Event, S: State<E>> Simulator<E,S> {

    pub fn new(state: S, events: Vec<E>) -> Simulator<E,S> {
        let mut sim = Simulator {
            time: 0,
            events: BinaryHeap::new(),
            state: state,
        };
        for e in events {
            sim.push_event(e);
        }
        sim
    }

    pub fn state(&mut self) -> &mut S {
        &mut self.state
    }

    pub fn time(&self) -> u64 {
        self.time
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

    pub fn run(&mut self, end_time: u64, stat_interval: u64) {
        let mut last_stat = 0;
        loop {
            match self.pop_event() {
                None => break,
                Some(event) => {
                    let time = event.time();
                    if time >= end_time { break; }
                    let stat_time = time / stat_interval * stat_interval;
                    if stat_time > last_stat {
                        last_stat = stat_time;
                        self.state.statistics(stat_time);
                    }
                    for e in self.state.handle(event) {
                        self.push_event(e);
                    }
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
}

