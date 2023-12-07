use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

static INPUT: &str = include_str!("../../../input/day07");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    input
        .lines()
        .map(Hand::from)
        .sorted()
        .rev()
        .enumerate()
        .map(|(r, h)| h.bid * (r + 1))
        .sum()
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    raw: String,
    num: (usize, usize, usize, usize, usize),
    bid: usize,
    strength: usize,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (cards, bid) = value.split_once(' ').unwrap();

        let tr8 = [
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ]
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i))
        .collect::<HashMap<_, _>>();

        let num = cards.chars().map(|c| *tr8.get(&c).unwrap()).collect_vec();

        let g = cards
            .chars()
            .counts()
            .into_iter()
            .map(|(c, i)| i)
            .sorted()
            .rev()
            .collect_vec();

        let strength = match g.len() {
            1 => 1,
            2 => match (g[0], g[1]) {
                (4, 1) => 2,
                (3, 2) => 3,
                _ => panic!(),
            },
            3 => match (g[0], g[1], g[2]) {
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

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 6440);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
