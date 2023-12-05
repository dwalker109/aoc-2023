use std::collections::HashMap;
use std::ops::Range;

static INPUT: &str = include_str!("../../../input/day05");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let (seeds, maps) = parse(input);

    seeds
        .iter()
        .map(|&s| {
            maps.iter().fold(s, |acc, cur| {
                for sm in cur {
                    if let Some(result) = sm.map(acc) {
                        return result;
                    }
                }

                acc
            })
        })
        .min()
        .unwrap()
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

fn parse(input: &str) -> (Vec<usize>, Vec<Vec<ConvMap>>) {
    let mut sections = input.split("\n\n");

    let seeds = sections
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let maps = sections
        .map(|s| s.lines().skip(1).map(ConvMap::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (seeds, maps)
}

struct ConvMap(Range<usize>, isize);

impl From<&str> for ConvMap {
    fn from(value: &str) -> Self {
        let mut vals = value.splitn(3, ' ');
        let dest_start = vals.next().unwrap().parse::<usize>().unwrap();
        let src_start = vals.next().unwrap().parse::<usize>().unwrap();
        let len = vals.next().unwrap().parse::<usize>().unwrap();

        let src_range = (src_start..src_start + len);
        let offset = dest_start as isize - src_start as isize;

        Self(src_range, offset)
    }
}

impl ConvMap {
    fn map(&self, src: usize) -> Option<usize> {
        if self.0.contains(&src) {
            return Some((src as isize + self.1) as usize);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 35);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
