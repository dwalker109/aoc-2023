use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../../../input/day03");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let symbols = parse_symbols(input);
    let numbers = parse_numbers(input);

    numbers
        .iter()
        .filter(|pn| pn.adjacents.iter().any(|xy| symbols.contains_key(xy)))
        .map(|pn| pn.num)
        .sum()
}

fn part2(input: &'static str) -> Answer {
    let gears = parse_symbols(input)
        .into_iter()
        .filter(|(_, c)| *c == '*')
        .collect::<HashMap<_, _>>();
    let numbers = parse_numbers(input);

    gears
        .iter()
        .filter_map(|(xy, _)| {
            let pns = numbers
                .iter()
                .filter(|pn| pn.adjacents.contains(xy))
                .collect::<Vec<_>>();

            if pns.len() == 2 {
                Some(pns.iter().map(|pn| pn.num).product::<usize>())
            } else {
                None
            }
        })
        .sum()
}

fn parse_symbols(input: &str) -> HashMap<Xy, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                (!c.is_numeric() && c != '.')
                    .then_some((Xy(x.try_into().unwrap(), y.try_into().unwrap()), c))
            })
        })
        .collect()
}

fn parse_numbers(input: &str) -> Vec<MaybePartNumber> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            let seq = l
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_numeric())
                .collect::<HashMap<_, _>>();

            let mut collected = Vec::new();
            let mut curr_seq = Vec::new();
            let mut curr_val = String::new();

            for x in 0_usize..l.len() {
                if let Some(c) = seq.get(&x) {
                    curr_seq.push(Xy(x.try_into().unwrap(), y.try_into().unwrap()));
                    curr_val.push(*c);

                    if !seq.contains_key(&(x + 1)) {
                        let adjacents = curr_seq
                            .iter()
                            .flat_map(|xy| xy.neighbours())
                            .collect::<HashSet<_>>();

                        collected.push(MaybePartNumber {
                            num: curr_val.parse().unwrap(),
                            adjacents,
                        });

                        curr_seq.clear();
                        curr_val.clear();
                    }
                }
            }

            collected
        })
        .collect()
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Xy(isize, isize);

impl Xy {
    pub fn neighbours(&self) -> [Xy; 8] {
        let Xy(x, y) = *self;
        [
            Xy(x - 1, y - 1),
            Xy(x, y - 1),
            Xy(x + 1, y - 1),
            Xy(x - 1, y),
            Xy(x + 1, y),
            Xy(x - 1, y + 1),
            Xy(x, y + 1),
            Xy(x + 1, y + 1),
        ]
    }
}

struct MaybePartNumber {
    num: usize,
    adjacents: HashSet<Xy>,
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 4361);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 467835);
    }
}
