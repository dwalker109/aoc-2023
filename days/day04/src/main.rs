use std::cmp::max;

static INPUT: &str = include_str!("../../../input/day04");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    input
        .lines()
        .map(|l| {
            let (_, body) = l.split_once(':').unwrap();
            let (winning, have) = body.split_once('|').unwrap();
            let winning = winning
                .split_ascii_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let have = have
                .split_ascii_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>();

            have.iter().fold(0, |acc, curr| {
                if winning.contains(curr) {
                    max(1, acc * 2)
                } else {
                    acc
                }
            })
        })
        .sum()
}

fn part2(input: &'static str) -> Answer {
    let mut cards = input
        .lines()
        .map(|l| {
            let (_, body) = l.split_once(':').unwrap();
            let (winning, have) = body.split_once('|').unwrap();
            let winning = winning
                .split_ascii_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let have = have
                .split_ascii_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>();

            (winning, have, 1_usize)
        })
        .collect::<Vec<_>>();

    for i in 0..cards.len() {
        let (winning, have, q) = &cards[i];
        let q = *q;
        let w = have.iter().fold(
            0_usize,
            |acc, curr| {
                if winning.contains(curr) {
                    acc + 1
                } else {
                    acc
                }
            },
        );

        for (_, _, n) in cards[i + 1..=i + w].iter_mut() {
            *n += q;
        }
    }

    cards.iter().map(|c| c.2).sum()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 30);
    }
}
