use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char as nomchar};
use nom::sequence::{delimited, separated_pair, tuple};
use num_integer::Integer;
use std::collections::HashMap;

static INPUT: &str = include_str!("../../../input/day08");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let steps = parse(input);
    steps.walk("AAA", |k| k == "ZZZ")
}

fn part2(input: &'static str) -> Answer {
    let steps = parse(input);
    steps
        .1
        .iter()
        .filter_map(|(&k, _)| k.ends_with('A').then_some(k))
        .map(|k| steps.walk(k, |k| k.ends_with('Z')))
        .reduce(|acc, curr| curr.lcm(&acc))
        .unwrap()
}

fn parse(input: &'static str) -> Steps {
    let (dirs, steps) = input.split_once("\n\n").unwrap();

    let steps = steps
        .lines()
        .map(|l| {
            let (_, (k, _, v)) = tuple((
                alphanumeric1::<_, nom::error::Error<_>>,
                tag(" = "),
                delimited(
                    nomchar('('),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    nomchar(')'),
                ),
            ))(l)
            .unwrap();
            (k, v)
        })
        .collect::<HashMap<_, _>>();

    Steps(dirs.chars().collect(), steps)
}

struct Steps(
    Vec<char>,
    HashMap<&'static str, (&'static str, &'static str)>,
);

impl Steps {
    fn walk(&self, start_from: &str, at_end: fn(&str) -> bool) -> usize {
        let mut dirs = self.0.iter().cycle();
        let mut next = self.1.get_key_value(start_from).unwrap();

        let mut n = 0_usize;

        loop {
            let (&key, &(l, r)) = next;

            if at_end(key) {
                return n;
            }

            n += 1;

            next = self
                .1
                .get_key_value(match dirs.next().unwrap() {
                    'L' => l,
                    'R' => r,
                    _ => panic!(),
                })
                .unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT_A: &str = include_str!("../input_test_a");
    static INPUT_B: &str = include_str!("../input_test_b");
    static INPUT_C: &str = include_str!("../input_test_c");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT_A), 2);
        assert_eq!(super::part1(INPUT_B), 6);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT_C), 6);
    }
}
