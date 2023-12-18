use crate::Dir;
use std::collections::{HashMap, VecDeque};

pub fn parse(input: &'static str) -> Vec<DigInst> {
    input.lines().map(DigInst::from).collect()
}

pub struct Lagoon(HashMap<Xy, &'static str>, Xy);

impl Lagoon {
    pub fn new() -> Self {
        Self(HashMap::new(), Xy(0, 0))
    }

    pub fn dig(&mut self, instr: &[DigInst]) {
        let mut curr = self.1;

        for i in instr {
            for _ in 0..i.len {
                curr = curr.next(&i.dir);
                self.0.insert(curr, i.hex);
            }
        }
    }

    pub fn fill(&mut self) {
        let min = self.0.keys().min().unwrap();
        let max = self.0.keys().max().unwrap();

        let mut pos = VecDeque::from(vec![min.next(&Dir::Right).next(&Dir::Down)]);
        pos.reserve((min.0..max.0).len() * (min.1..max.1).len());

        while let Some(p) = pos.pop_front() {
            if self.0.insert(p, "").is_none() {
                for n in p.adj().iter().filter(|&xy| !self.0.contains_key(xy)) {
                    pos.push_back(*n)
                }
            }
        }
    }

    pub fn capacity(&self) -> usize {
        self.0.len()
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd, Debug)]
struct Xy(isize, isize);

impl Xy {
    pub fn next(&self, dir: &Dir) -> Self {
        let Xy(x, y) = *self;

        match dir {
            Dir::Up => Xy(x, y - 1),
            Dir::Down => Xy(x, y + 1),
            Dir::Left => Xy(x - 1, y),
            Dir::Right => Xy(x + 1, y),
        }
    }

    pub fn adj(&self) -> [Xy; 4] {
        [
            self.next(&Dir::Up),
            self.next(&Dir::Down),
            self.next(&Dir::Left),
            self.next(&Dir::Right),
        ]
    }
}

pub struct DigInst {
    dir: Dir,
    len: isize,
    hex: &'static str,
}

impl From<&'static str> for DigInst {
    fn from(value: &'static str) -> Self {
        let mut parts = value.splitn(3, ' ');

        let dir = match parts.next().unwrap() {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => unimplemented!(),
        };

        let len = parts.next().unwrap().parse().unwrap();

        let hex = parts
            .next()
            .unwrap()
            .trim_start_matches('(')
            .trim_end_matches(')');

        Self { dir, len, hex }
    }
}
