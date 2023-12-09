#![feature(array_windows)]

static INPUT: &str = include_str!("../../../input/day09");

type Answer = isize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    to_sequences(input)
        .iter()
        .map(|s| s.iter().rfold(0, |prev, [_, max]| prev + max))
        .sum()
}
fn part2(input: &'static str) -> Answer {
    to_sequences(input)
        .iter()
        .map(|s| s.iter().rfold(0, |prev, [min, _]| min - prev))
        .sum()
}

fn to_sequences(input: &str) -> Vec<Vec<[isize; 2]>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .filter_map(|n| n.parse::<isize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|mut seq| {
            let mut results = Vec::with_capacity(seq.len());
            results.push([*seq.first().unwrap(), *seq.last().unwrap()]);

            while seq.iter().any(|n| *n != 0) {
                seq = seq.array_windows::<2>().fold(vec![], |mut acc, [l, r]| {
                    acc.push(r - l);
                    acc
                });

                results.push([*seq.first().unwrap(), *seq.last().unwrap()]);
            }

            results
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
