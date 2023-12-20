use crate::ipc::*;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::Sender;

pub(crate) trait Communicate {
    fn send(&self, queue: &mut VecDeque<(Pulse, ModId, ModId)>, pulse: Pulse) {
        for to_id in self.dest() {
            queue.push_back((pulse, self.id().clone(), to_id.clone()));
        }
    }

    fn rcv(&mut self, queue: &mut VecDeque<(Pulse, ModId, ModId)>, pulse: Pulse, from: &ModId) {
        eprintln!("Receiving is a no-op by default!");
    }

    fn id(&self) -> &ModId;
    fn dest(&self) -> &[ModId];
}

#[derive(Hash, Eq, PartialEq)]
enum State {
    On,
    Off,
}

pub type ModId = String;

pub struct FlipFlop {
    id: ModId,
    state: State,
    dest: Vec<ModId>,
}

impl FlipFlop {
    pub fn new(id: ModId, dest: Vec<ModId>) -> Self {
        Self {
            id,
            state: State::Off,
            dest,
        }
    }
}

impl Communicate for FlipFlop {
    fn rcv(&mut self, queue: &mut VecDeque<(Pulse, ModId, ModId)>, pulse: Pulse, _: &ModId) {
        if matches!(pulse, Pulse::Low) {
            match self.state {
                State::On => {
                    self.state = State::Off;
                    self.send(queue, Pulse::Low);
                }
                State::Off => {
                    self.state = State::On;
                    self.send(queue, Pulse::High);
                }
            }
        }
    }

    fn id(&self) -> &ModId {
        &self.id
    }

    fn dest(&self) -> &[ModId] {
        &self.dest
    }
}

pub struct Conjunction {
    id: ModId,
    state: State,
    dest: Vec<ModId>,
    mem: HashMap<ModId, Pulse>,
}

impl Conjunction {
    pub fn new(id: ModId, dest: Vec<ModId>, mem_src: &[ModId]) -> Self {
        Self {
            id,
            state: State::Off,
            dest,
            mem: mem_src.iter().map(|id| (id.clone(), Pulse::Low)).collect(),
        }
    }
}

impl Communicate for Conjunction {
    fn rcv(&mut self, queue: &mut VecDeque<(Pulse, ModId, ModId)>, pulse: Pulse, from: &ModId) {
        *self.mem.get_mut(from).unwrap() = pulse;

        if self.mem.values().all(|p| matches!(p, Pulse::High)) {
            self.send(queue, Pulse::Low);
        } else {
            self.send(queue, Pulse::High);
        }
    }

    fn id(&self) -> &ModId {
        &self.id
    }

    fn dest(&self) -> &[ModId] {
        &self.dest
    }
}

pub struct Broadcast {
    id: ModId,
    dest: Vec<ModId>,
}

impl Broadcast {
    pub fn new(id: ModId, dest: Vec<ModId>) -> Self {
        Self { id, dest }
    }
}

impl Communicate for Broadcast {
    fn rcv(&mut self, queue: &mut VecDeque<(Pulse, ModId, ModId)>, pulse: Pulse, from: &ModId) {
        self.send(queue, pulse)
    }

    fn id(&self) -> &ModId {
        &self.id
    }

    fn dest(&self) -> &[ModId] {
        &self.dest
    }
}

pub struct Button {
    id: ModId,
    dest: Vec<ModId>,
}

impl Communicate for Button {
    fn id(&self) -> &ModId {
        &self.id
    }

    fn dest(&self) -> &[ModId] {
        &self.dest
    }
}

impl Button {
    pub fn new() -> Self {
        Self {
            id: "button".into(),
            dest: vec!["broadcaster".into()],
        }
    }
}
