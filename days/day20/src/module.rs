use crate::ipc::*;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub(crate) trait Communicate {
    fn send(&self, pulse: Pulse) {
        for to_id in self.dest() {
            self.sender()
                .send((pulse, self.id().clone(), to_id.clone()))
                .unwrap();
        }
    }

    fn rcv(&mut self, pulse: Pulse, from: &ModId) {
        eprintln!("Receiving is a no-op by default!");
    }

    fn id(&self) -> &ModId;
    fn dest(&self) -> &[ModId];
    fn sender(&self) -> &Sender<(Pulse, ModId, ModId)>;
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
    sender: Sender<(Pulse, ModId, ModId)>,
}

impl FlipFlop {
    pub fn new(id: ModId, dest: Vec<ModId>, sender: Sender<(Pulse, ModId, ModId)>) -> Self {
        Self {
            id,
            state: State::Off,
            dest,
            sender,
        }
    }
}

impl Communicate for FlipFlop {
    fn rcv(&mut self, pulse: Pulse, _: &ModId) {
        if matches!(pulse, Pulse::Low) {
            match self.state {
                State::On => {
                    self.state = State::Off;
                    self.send(Pulse::Low);
                }
                State::Off => {
                    self.state = State::On;
                    self.send(Pulse::High);
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

    fn sender(&self) -> &Sender<(Pulse, ModId, ModId)> {
        &self.sender
    }
}

pub struct Conjunction {
    id: ModId,
    state: State,
    dest: Vec<ModId>,
    sender: Sender<(Pulse, ModId, ModId)>,
    mem: HashMap<ModId, Pulse>,
}

impl Conjunction {
    pub fn new(
        id: ModId,
        dest: Vec<ModId>,
        sender: Sender<(Pulse, ModId, ModId)>,
        mem_src: &[ModId],
    ) -> Self {
        Self {
            id,
            state: State::Off,
            dest,
            sender,
            mem: mem_src.iter().map(|id| (id.clone(), Pulse::Low)).collect(),
        }
    }
}

impl Communicate for Conjunction {
    fn rcv(&mut self, pulse: Pulse, from: &ModId) {
        *self.mem.get_mut(from).unwrap() = pulse;

        if self.mem.values().all(|p| matches!(p, Pulse::High)) {
            self.send(Pulse::Low);
        } else {
            self.send(Pulse::High);
        }
    }

    fn id(&self) -> &ModId {
        &self.id
    }

    fn dest(&self) -> &[ModId] {
        &self.dest
    }

    fn sender(&self) -> &Sender<(Pulse, ModId, ModId)> {
        &self.sender
    }
}

pub struct Broadcast {
    id: ModId,
    dest: Vec<ModId>,
    sender: Sender<(Pulse, ModId, ModId)>,
}

impl Broadcast {
    pub fn new(id: ModId, dest: Vec<ModId>, sender: Sender<(Pulse, ModId, ModId)>) -> Self {
        Self { id, dest, sender }
    }
}

impl Communicate for Broadcast {
    fn rcv(&mut self, pulse: Pulse, from: &ModId) {
        self.send(pulse)
    }

    fn id(&self) -> &ModId {
        &self.id
    }

    fn dest(&self) -> &[ModId] {
        &self.dest
    }

    fn sender(&self) -> &Sender<(Pulse, ModId, ModId)> {
        &self.sender
    }
}

pub struct Button {
    id: ModId,
    dest: Vec<ModId>,
    sender: Sender<(Pulse, ModId, ModId)>,
}

impl Communicate for Button {
    fn id(&self) -> &ModId {
        &self.id
    }

    fn dest(&self) -> &[ModId] {
        &self.dest
    }

    fn sender(&self) -> &Sender<(Pulse, ModId, ModId)> {
        &self.sender
    }
}

impl Button {
    pub fn new(sender: Sender<(Pulse, ModId, ModId)>) -> Self {
        Self {
            id: "button".into(),
            dest: vec!["broadcaster".into()],
            sender,
        }
    }
}
