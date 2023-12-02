static INPUT: &str = include_str!("../../../input/day02");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    input
        .lines()
        .filter_map(|l| {
            let (id, rest) = parse_preamble(l);
            let mut rounds = parse_round(rest);

            if rounds.all(|c| match c {
                Cubes::Red(q) if q > 12 => false,
                Cubes::Green(q) if q > 13 => false,
                Cubes::Blue(q) if q > 14 => false,
                _ => true,
            }) {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &'static str) -> Answer {
    input
        .lines()
        .map(|l| {
            let (_, rest) = parse_preamble(l);
            let rounds = parse_round(rest).collect::<Vec<_>>();

            let red: usize = rounds
                .iter()
                .filter(|c| matches!(c, Cubes::Red(..)))
                .max()
                .unwrap()
                .into();
            let green: usize = rounds
                .iter()
                .filter(|c| matches!(c, Cubes::Green(..)))
                .max()
                .unwrap()
                .into();
            let blue: usize = rounds
                .iter()
                .filter(|c| matches!(c, Cubes::Blue(..)))
                .max()
                .unwrap()
                .into();

            red * green * blue
        })
        .sum()
}

#[derive(Ord, PartialOrd, PartialEq, Eq)]
enum Cubes {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl From<&Cubes> for usize {
    fn from(val: &Cubes) -> Self {
        match val {
            Cubes::Red(q) => *q,
            Cubes::Green(q) => *q,
            Cubes::Blue(q) => *q,
        }
    }
}

fn parse_preamble(input: &str) -> (usize, &str) {
    let (game, rest) = input.split_once(':').unwrap();
    let (_, id) = game.split_once(' ').unwrap();

    (id.parse().unwrap(), rest)
}

fn parse_round(input: &str) -> impl Iterator<Item = Cubes> + '_ {
    input.split(';').flat_map(|c| {
        c.split(',').map(|r| {
            let (n, c) = r.trim().split_once(' ').unwrap();
            match c.as_bytes() {
                b"red" | b"red," => Cubes::Red(n.parse::<usize>().unwrap()),
                b"green" | b"green," => Cubes::Green(n.parse::<usize>().unwrap()),
                b"blue" | b"blue," => Cubes::Blue(n.parse::<usize>().unwrap()),
                _ => panic!("bad parse! {c}"),
            }
        })
    })
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 8);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 2286);
    }
}
