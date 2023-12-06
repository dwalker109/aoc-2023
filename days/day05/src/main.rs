#![feature(array_chunks)]

use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashSet;
use std::ops::Range;

static INPUT: &str = include_str!("../../../input/day05");

type Answer = isize;

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
                    if let Some(result) = sm.map_one(acc) {
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
    let (seeds, maps) = parse(input);

    // let seeds = [82, 1];

    seeds
        .array_chunks()
        .map(|[start, len]| *start..(start + len))
        .flat_map(|r| {
            maps.iter()
                .fold([r].into_iter().collect::<HashSet<_>>(), |acc, cur| {
                    let mut next = HashSet::new();
                    let mut unchecked = acc.into_iter().collect::<Vec<_>>();

                    for sm in cur {
                        let mut next_unchecked = Vec::new();

                        while let Some(sr) = unchecked.pop() {
                            let (mapped, unmapped) = sm.map_range(sr.clone());

                            if let Some(mapped) = mapped {
                                next.extend(mapped);
                            }

                            if let Some(unmapped) = unmapped {
                                next_unchecked.extend(unmapped);
                            }
                        }

                        unchecked = next_unchecked;
                    }

                    next.into_iter().chain(unchecked).collect()
                })
        })
        .map(|r| r.start)
        .min()
        .unwrap()
}

fn parse(input: &str) -> (Vec<isize>, Vec<Vec<ConvMap>>) {
    let mut sections = input.split("\n\n");

    let seeds = sections
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<isize>().ok())
        .collect::<Vec<_>>();

    let maps = sections
        .map(|s| s.lines().skip(1).map(ConvMap::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (seeds, maps)
}

struct ConvMap(Range<isize>, isize);

impl From<&str> for ConvMap {
    fn from(value: &str) -> Self {
        let mut vals = value.splitn(3, ' ');
        let dest_start = vals.next().unwrap().parse::<isize>().unwrap();
        let src_start = vals.next().unwrap().parse::<isize>().unwrap();
        let len = vals.next().unwrap().parse::<isize>().unwrap();

        let src_range = src_start..src_start + len;
        let offset = dest_start - src_start;

        Self(src_range, offset)
    }
}

impl ConvMap {
    fn map_one(&self, target: isize) -> Option<isize> {
        if self.0.contains(&target) {
            return Some(target + self.1);
        }

        None
    }

    fn map_range(
        &self,
        lookup: Range<isize>,
    ) -> (Option<Vec<Range<isize>>>, Option<Vec<Range<isize>>>) {
        let (map_from, offset) = (&self.0, self.1);

        let offset_range = |start: isize, end: isize| {
            let s = start + offset;
            let e = end + offset;

            s..e
        };

        match (
            lookup.start.cmp(&map_from.start),
            lookup.end.cmp(&map_from.start),
            lookup.start.cmp(&map_from.end),
            lookup.end.cmp(&map_from.end),
        ) {
            (Less, Less | Equal, _, _) => {
                //all before
                (None, Some(vec![lookup]))
            }
            (_, _, Greater | Equal, Greater) => {
                // all after
                (None, Some(vec![lookup]))
            }
            (Greater | Equal, _, _, Less | Equal) => {
                // all within
                (Some(vec![offset_range(lookup.start, lookup.end)]), None)
            }
            (Less, _, _, Less | Equal) => {
                // start before, end within
                (
                    Some(vec![offset_range(map_from.start, lookup.end)]),
                    Some(vec![lookup.start..map_from.start]),
                )
            }
            (Greater | Equal, _, _, Greater) => {
                // start within, end after
                (
                    Some(vec![offset_range(lookup.start, map_from.end)]),
                    Some(vec![map_from.end..lookup.end]),
                )
            }
            (Less, _, _, Greater) => {
                // start before, end after
                (
                    Some(vec![offset_range(map_from.start, map_from.end)]),
                    Some(vec![lookup.start..map_from.start, map_from.end..lookup.end]),
                )
            }
        }
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
        assert_eq!(super::part2(INPUT), 46);
    }
}
