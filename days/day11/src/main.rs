use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

static INPUT: &str = include_str!("../../../input/day11");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let (offset_x, offset_y) = offsets(input);
    let starchart = parse(input);

    starchart
        .keys()
        .combinations(2)
        .map(|p| {
            let (ax, ay) = starchart[p[0]];
            let (bx, by) = starchart[p[1]];

            let (ax, ay) = (offset_x[&ax], offset_y[&ay]);
            let (bx, by) = (offset_x[&bx], offset_y[&by]);

            (ax.abs_diff(bx)) + ay.abs_diff(by)
        })
        .sum()
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

fn offsets(input: &str) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    let col_skip = width + 1; // Account for the EOL
    let offsets_x = (0..col_skip)
        .filter(|&x| input.chars().skip(x).step_by(col_skip).all(|c| c == '.'))
        .collect::<BTreeSet<_>>();

    let offsets_y = input
        .lines()
        .enumerate()
        .filter_map(|(y, l)| l.chars().all(|c| c == '.').then_some(y))
        .collect::<BTreeSet<_>>();

    let offsets_x = (0..width)
        .map(|x| {
            let o = offsets_x.iter().filter(|&v| *v < x).count();
            (x, x + o)
        })
        .collect();

    let offsets_y = (0..height)
        .map(|y| {
            let o = offsets_y.iter().filter(|&v| *v < y).count();
            (y, y + o)
        })
        .collect();

    (offsets_x, offsets_y)
}

fn parse(input: &str) -> HashMap<usize, (usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
        })
        .enumerate()
        .collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 374);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
