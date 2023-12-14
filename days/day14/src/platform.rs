use super::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::mem::swap;

pub struct Platform {
    square_rocks: HashSet<Xy>,
    pub round_rocks: HashSet<Xy>,
    pub width: isize,
    pub height: isize,
}

impl From<&str> for Platform {
    fn from(value: &str) -> Self {
        let mut square_rocks = HashSet::new();
        let mut round_rocks = HashSet::new();

        for (y, l) in value.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => {
                        square_rocks.insert(Xy(x as isize, y as isize));
                    }
                    'O' => {
                        round_rocks.insert(Xy(x as isize, y as isize));
                    }
                    _ => continue,
                }
            }
        }

        let width = value.lines().next().unwrap().chars().count() as isize;
        let height = value.lines().count() as isize;

        Platform {
            square_rocks,
            round_rocks,
            width,
            height,
        }
    }
}

impl Platform {
    pub fn tilt_north(&mut self) {
        let mut next = HashSet::with_capacity(self.round_rocks.len());

        for x in 0..self.height {
            let obs = self
                .square_rocks
                .iter()
                .filter_map(|Xy(x2, y2)| (x == *x2).then_some(*y2))
                .sorted();
            let obs = [-1].into_iter().chain(obs.chain([self.height].into_iter()));

            let rocks = self
                .round_rocks
                .iter()
                .filter_map(|Xy(x2, y2)| (x == *x2).then_some(*y2))
                .collect::<Vec<_>>();

            for (a, b) in obs.tuple_windows() {
                let q = rocks.iter().filter(|&&y| y > a && y < b).count() as isize;
                for n in 1..=q {
                    next.insert(Xy(x, a + n));
                }
            }
        }

        swap(&mut self.round_rocks, &mut next);
    }

    pub fn tilt_south(&mut self) {
        let mut next = HashSet::with_capacity(self.round_rocks.len());

        for x in 0..self.height {
            let obs = self
                .square_rocks
                .iter()
                .filter_map(|Xy(x2, y2)| (x == *x2).then_some(*y2))
                .sorted();
            let obs = [-1].into_iter().chain(obs.chain([self.height].into_iter()));

            let rocks = self
                .round_rocks
                .iter()
                .filter_map(|Xy(x2, y2)| (x == *x2).then_some(*y2))
                .collect::<Vec<_>>();

            for (a, b) in obs.tuple_windows() {
                let q = rocks.iter().filter(|&&y| y > a && y < b).count() as isize;
                for n in 1..=q {
                    next.insert(Xy(x, b - n));
                }
            }
        }

        swap(&mut self.round_rocks, &mut next);
    }

    pub fn tilt_west(&mut self) {
        let mut next = HashSet::with_capacity(self.round_rocks.len());

        for y in 0..self.width {
            let obs = self
                .square_rocks
                .iter()
                .filter_map(|Xy(x2, y2)| (y == *y2).then_some(*x2))
                .sorted();
            let obs = [-1].into_iter().chain(obs.chain([self.width].into_iter()));

            let rocks = self
                .round_rocks
                .iter()
                .filter_map(|Xy(x2, y2)| (y == *y2).then_some(*x2))
                .collect::<Vec<_>>();

            for (a, b) in obs.tuple_windows() {
                let q = rocks.iter().filter(|&&x| x > a && x < b).count() as isize;
                for n in 1..=q {
                    next.insert(Xy(a + n, y));
                }
            }
        }

        swap(&mut self.round_rocks, &mut next);
    }

    pub fn tilt_east(&mut self) {
        let mut next = HashSet::with_capacity(self.round_rocks.len());

        for y in 0..self.width {
            let obs = self
                .square_rocks
                .iter()
                .filter_map(|Xy(x2, y2)| (y == *y2).then_some(*x2))
                .sorted();
            let obs = [-1].into_iter().chain(obs.chain([self.width].into_iter()));

            let rocks = self
                .round_rocks
                .iter()
                .filter_map(|Xy(x2, y2)| (y == *y2).then_some(*x2))
                .collect::<Vec<_>>();

            for (a, b) in obs.tuple_windows() {
                let q = rocks.iter().filter(|&&x| x > a && x < b).count() as isize;
                for n in 1..=q {
                    next.insert(Xy(b - n, y));
                }
            }
        }

        swap(&mut self.round_rocks, &mut next);
    }

    pub fn total_load(&self) -> usize {
        self.round_rocks
            .iter()
            .map(|Xy(_, y)| y.abs_diff(self.height))
            .sum()
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.width {
            for x in 0..self.height {
                if let Some(_) = self.square_rocks.get(&Xy(x, y)) {
                    write!(f, "#").ok();
                } else if let Some(_) = self.round_rocks.get(&Xy(x, y)) {
                    write!(f, "0").ok();
                } else {
                    write!(f, ".").ok();
                }
            }
            writeln!(f, "").ok();
        }

        writeln!(f, "")
    }
}
