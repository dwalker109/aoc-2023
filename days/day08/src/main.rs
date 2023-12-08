use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char};
use nom::sequence::{delimited, separated_pair, tuple};
use num_integer::{lcm, Integer};
use std::collections::HashMap;

static INPUT: &str = include_str!("../../../input/day08");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let (mut dirs, steps) = parse(input);
    let mut next = steps.get_key_value("AAA").unwrap();

    let mut n = 0;
    loop {
        let (&key, &(l, r)) = next;

        if key == "ZZZ" {
            return n;
        }

        n += 1;
        next = steps
            .get_key_value(match dirs.next().unwrap() {
                'L' => l,
                'R' => r,
                _ => panic!(),
            })
            .unwrap();
    }

    unreachable!()
}

fn part2(input: &'static str) -> Answer {
    let (mut dirs, steps) = parse(input);
    let mut starts = steps.iter().filter(|&((&k, _))| k.ends_with('A'));

    let each = starts
        .map(|(&k, _)| {
            let mut next = steps.get_key_value(k).unwrap();

            let mut n = 0_usize;
            loop {
                let (&key, &(l, r)) = next;

                if key.ends_with('Z') {
                    return n;
                }

                n += 1;
                next = steps
                    .get_key_value(match dirs.next().unwrap() {
                        'L' => l,
                        'R' => r,
                        _ => panic!(),
                    })
                    .unwrap();
            }
        })
        .collect::<Vec<_>>();

    each.into_iter().reduce(|acc, curr| curr.lcm(&acc)).unwrap()
}

fn parse(input: &str) -> (impl Iterator<Item = char> + '_, HashMap<&str, (&str, &str)>) {
    let (dirs, steps) = input.split_once("\n\n").unwrap();

    let dirs = dirs.chars().cycle();
    let steps = steps
        .lines()
        .map(|l| {
            let (_, (k, _, v)) = tuple((
                alphanumeric1::<_, nom::error::Error<_>>,
                tag(" = "),
                delimited(
                    char('('),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    char(')'),
                ),
            ))(l)
            .unwrap();
            (k, v)
        })
        .collect::<HashMap<_, _>>();

    (dirs, steps)
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
