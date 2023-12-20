use crate::module::*;
use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone)]
pub enum Pulse {
    High,
    Low,
}

pub struct Repo {
    modules: HashMap<ModId, Box<dyn Communicate>>,
    queue: VecDeque<(Pulse, ModId, ModId)>,
    pub hi_qty: usize,
    pub lo_qty: usize,
}

impl Repo {
    pub fn new(modules: HashMap<ModId, Box<dyn Communicate>>) -> Self {
        Self {
            modules,
            queue: VecDeque::new(),
            hi_qty: 0,
            lo_qty: 0,
        }
    }

    pub fn push_the_button(&mut self) {
        let button = self.modules.get_mut("button").unwrap();
        button.send(&mut self.queue, Pulse::Low);

        while let Some((pulse, from_id, to_id)) = self.queue.pop_front() {
            match pulse {
                Pulse::High => self.hi_qty += 1,
                Pulse::Low => self.lo_qty += 1,
            }

            if let Some(m) = self.modules.get_mut(&to_id) {
                m.rcv(&mut self.queue, pulse, &from_id);
            }
        }
    }
}
