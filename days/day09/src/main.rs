#![feature(array_windows)]

static INPUT: &str = include_str!("../../../input/day09");

type Answer = isize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    to_sequences(input)
        .iter()
        .map(|s| {
            s.iter()
                .map(|l| l.last().unwrap().to_owned())
                .rfold(0, |prev, curr| prev + curr)
        })
        .sum()
}
fn part2(input: &'static str) -> Answer {
    to_sequences(input)
        .iter()
        .map(|s| {
            s.iter()
                .map(|l| l.first().unwrap().to_owned())
                .rfold(0, |prev, curr| curr - prev)
        })
        .sum()
}

fn to_sequences(input: &str) -> Vec<Vec<Vec<isize>>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .filter_map(|n| n.parse::<isize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|seq| {
            let mut to_zero = vec![seq];
            to_zero.reserve(to_zero[0].len());

            while to_zero.last().unwrap().iter().any(|n| *n != 0) {
                let next =
                    to_zero
                        .last()
                        .unwrap()
                        .array_windows::<2>()
                        .fold(vec![], |mut acc, [l, r]| {
                            acc.push(r - l);
                            acc
                        });

                to_zero.push(next)
            }

            to_zero
        })
        .collect()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 114);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 2);
    }
}
