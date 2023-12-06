static INPUT: &str = include_str!("../../../input/day06");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let (q, dur, dist) = &parse_1(input);

    (0..*q)
        .map(|i| {
            let dur = dur[i];
            let dist = dist[i];

            (0..=dur)
                .map(|x| (dur - x) * x)
                .filter(|x| *x > dist)
                .count()
        })
        .product()
}

fn part2(input: &'static str) -> Answer {
    let (dur, dist) = parse_2(input);

    (0..=dur)
        .map(|x| (dur - x) * x)
        .filter(|x| *x > dist)
        .count()
}

fn parse_1(input: &str) -> (usize, Vec<usize>, Vec<usize>) {
    let rows = input
        .lines()
        .map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .split_ascii_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let q = rows[0].len();

    (q, rows[0].to_owned(), rows[1].to_owned())
}

fn parse_2(input: &str) -> (usize, usize) {
    let rows = input
        .lines()
        .filter_map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .split_ascii_whitespace()
                .collect::<String>()
                .parse::<usize>()
                .ok()
        })
        .collect::<Vec<_>>();

    (rows[0].to_owned(), rows[1].to_owned())
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 288);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 71503);
    }
}
