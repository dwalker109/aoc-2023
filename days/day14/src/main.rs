use crate::platform::Platform;

use std::fmt::{Display, Formatter};



static INPUT: &str = include_str!("../../../input/day14");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let mut p = parse(input);
    p.tilt_north();
    p.total_load()
}

fn part2(input: &'static str) -> Answer {
    static TARGET: usize = 1000000000;

    let mut p = parse(input);
    let mut cycles = cycles::Cycles::new();

    let (start, len, elapsed) = loop {
        p.tilt_north();
        p.tilt_west();
        p.tilt_south();
        p.tilt_east();

        if let Some(found) = cycles.push(p.total_load()) {
            break found;
        }
    };

    // Yuck. The interplay between elapsed, start and the way we calc fast forward still confuses me.
    let skip = TARGET - elapsed + start - (TARGET - elapsed).rem_euclid(len);

    for _ in skip..TARGET {
        p.tilt_north();
        p.tilt_west();
        p.tilt_south();
        p.tilt_east();
    }

    p.total_load()
}

fn parse(input: &str) -> Platform {
    Platform::from(input)
}

mod platform;

mod cycles {
    pub struct Cycles(Vec<usize>, Vec<Vec<usize>>);

    impl Cycles {
        pub fn new() -> Self {
            Cycles(Vec::new(), Vec::new())
        }

        pub fn push(&mut self, val: usize) -> Option<(usize, usize, usize)> {
            const MIN_CYCLE_SIZE: usize = 5;

            let (all, groups) = (&mut self.0, &mut self.1);

            all.push(val);
            groups.iter_mut().for_each(|g| g.push(val));
            groups.push(vec![val]);

            for g in groups.iter().filter(|g| g.len() > MIN_CYCLE_SIZE) {
                let halfway = g.len() / 2;
                if g[..halfway] == g[halfway..] {
                    return Some((all.len() - g.len(), halfway, all.len()));
                }
            }

            None
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Xy(isize, isize);

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 136);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 64);
    }
}
