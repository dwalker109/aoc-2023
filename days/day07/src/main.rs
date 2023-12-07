use itertools::Itertools;

static INPUT: &str = include_str!("../../../input/day07");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    input
        .lines()
        .map(reg::Hand::from)
        .sorted()
        .rev()
        .enumerate()
        .map(|(r, h)| h.bid * (r + 1))
        .sum()
}

fn part2(input: &'static str) -> Answer {
    input
        .lines()
        .map(jkr::Hand::from)
        .sorted()
        .rev()
        .enumerate()
        .map(|(r, h)| h.bid * (r + 1))
        .sum()
}

mod jkr;
mod reg;

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
