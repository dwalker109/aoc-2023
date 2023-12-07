use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap};

static INPUT: &str = include_str!("../../../input/day07");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    input
        .lines()
        .map(|l| Hand::from((l, Mode::Reg)))
        .sorted()
        .rev()
        .enumerate()
        .map(|(r, h)| h.bid * (r + 1))
        .sum()
}

fn part2(input: &'static str) -> Answer {
    input
        .lines()
        .map(|l| Hand::from((l, Mode::Jkr)))
        .sorted()
        .rev()
        .enumerate()
        .map(|(r, h)| h.bid * (r + 1))
        .sum()
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    raw: String,
    num: (usize, usize, usize, usize, usize),
    pub bid: usize,
    strength: usize,
}

impl From<(&str, Mode)> for Hand {
    fn from((value, mode): (&str, Mode)) -> Self {
        let (cards, bid) = value.split_once(' ').unwrap();

        let rel_s = mode.relative_strength();
        let num = cards.chars().map(|c| *rel_s.get(&c).unwrap()).collect_vec();
        let gr = mode.group(cards);

        let strength = match gr.len() {
            1 => 1,
            2 => match (gr[0], gr[1]) {
                (4, 1) => 2,
                (3, 2) => 3,
                _ => panic!(),
            },
            3 => match (gr[0], gr[1], gr[2]) {
                (3, 1, 1) => 4,
                (2, 2, 1) => 5,
                _ => panic!(),
            },
            4 => 6,
            5 => 7,
            _ => panic!(),
        };

        Self {
            raw: cards.to_string(),
            num: (num[0], num[1], num[2], num[3], num[4]),
            bid: bid.parse().unwrap(),
            strength,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.num.cmp(&other.num),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

enum Mode {
    Reg,
    Jkr,
}

impl Mode {
    fn relative_strength(&self) -> HashMap<char, usize> {
        match self {
            Mode::Reg => [
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ],

            Mode::Jkr => [
                'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
            ],
        }
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i))
        .collect::<HashMap<_, _>>()
    }

    fn group(&self, cards: &str) -> Vec<usize> {
        let mut grouped = cards.chars().counts();

        let jkrval = match self {
            Mode::Reg => 0,
            Mode::Jkr => grouped.remove(&'J').unwrap_or_default(),
        };

        let mut length_groups = grouped
            .into_iter()
            .map(|(_c, i)| i)
            .sorted()
            .rev()
            .collect_vec();

        match length_groups.is_empty() {
            true => length_groups.push(jkrval),
            false => length_groups[0] += jkrval,
        };

        length_groups
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 6440);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 5905);
    }
}
