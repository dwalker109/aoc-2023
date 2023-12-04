use std::cmp::max;

static INPUT: &str = include_str!("../../../input/day04");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    input
        .lines()
        .map(parse)
        .map(|(winning, have, _)| {
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
    let mut cards = input.lines().map(parse).collect::<Vec<_>>();

    for i in 0..cards.len() {
        let (winning, have, curr_q) = &cards[i];
        let curr_q = *curr_q;

        let wins = have.iter().fold(
            0_usize,
            |acc, curr| {
                if winning.contains(curr) {
                    acc + 1
                } else {
                    acc
                }
            },
        );

        for (_, _, next_q) in cards[i + 1..=i + wins].iter_mut() {
            *next_q += curr_q;
        }
    }

    cards.iter().map(|c| c.2).sum()
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>, usize) {
    let (_, body) = input.split_once(':').unwrap();
    let (winning, have) = body.split_once('|').unwrap();
    let winning = winning
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();
    let have = have
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    (winning, have, 1)
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
