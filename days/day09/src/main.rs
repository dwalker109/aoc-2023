#![feature(array_windows)]

static INPUT: &str = include_str!("../../../input/day09");

type Answer = isize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let vals = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .filter_map(|n| n.parse::<isize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    vals.into_iter()
        .map(|seq| {
            let mut to_zero = vec![seq.clone()];

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

                to_zero.push(next);
            }

            let finals = to_zero.into_iter().map(|l| l.last().unwrap().to_owned());

            finals.rfold(0, |prev, curr| prev + curr)
        })
        .sum()
}

fn part2(input: &'static str) -> Answer {
    todo!();
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
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
