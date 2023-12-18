static INPUT: &str = include_str!("../../../input/day18");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let instr = crate::part_1::parse(input);
    let mut lagoon = crate::part_1::Lagoon::new();

    lagoon.dig(&instr);
    lagoon.fill();

    lagoon.capacity()
}

fn part2(input: &'static str) -> Answer {
    let poly = part_2::parse(input);
    part_2::calc_area_with_trench(&poly)
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

mod part_1;
mod part_2;

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 62);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 952408144115);
    }
}
