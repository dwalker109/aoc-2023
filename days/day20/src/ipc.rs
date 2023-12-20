use crate::module::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    sync::{mpsc, Arc, RwLock},
    thread,
    thread::JoinHandle,
};

#[derive(Copy, Clone)]
pub enum Pulse {
    High,
    Low,
}

enum Msg {
    Pulse(Pulse, ModId, ModId),
    Report(Sender<usize>),
}

pub struct Repo {
    // modules: HashMap<ModId, Box<dyn Communicate>>,
    handle: JoinHandle<()>,
    pub sender: Sender<Msg>,
}

impl Repo {
    pub fn new(mut modules: HashMap<ModId, Box<dyn Communicate>>) -> Self {
        let (sender, receiver) = channel();

        let handle = thread::spawn(move || {
            let mut hist = Vec::new();

            while let Ok(msg) = receiver.recv() {
                match msg {
                    Msg::Pulse(p, from_id, to_id) => {
                        if let Some(module) = modules.get_mut(&to_id) {
                            module.rcv(p, &from_id);
                            hist.push(p);
                        }
                    }
                    Msg::Report(tx) => {
                        tx.send(hist.len()).ok();
                    }
                }
            }
        });

        Self { handle, sender }
    }

    pub fn push_the_button(&mut self) {
        let b = self.modules.get_mut("button".into()).unwrap();
        b.send(Pulse::Low);
    }

    pub fn fwd(&mut self, pulse: Pulse, from_id: &ModId, to_id: &ModId) {
        if let Some(module) = self.modules.get_mut(to_id) {
            module.rcv(pulse, from_id);
        }
    }

    pub fn cleanup(&mut self) {
        self.modules.clear();
    }
}
