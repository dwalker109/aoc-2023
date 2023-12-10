use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../../../input/day10");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let map = Map::from(input);
    map.loop_len() / 2
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Xy(isize, isize);

impl Xy {
    fn neighbouring(&self) -> [Xy; 8] {
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

#[derive(Clone, Copy)]
enum Pipe {
    VerticalNorthSouth,
    HorizontalEastWest,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Unknown,
    //     | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::VerticalNorthSouth),
            '-' => Ok(Self::HorizontalEastWest),
            'L' => Ok(Self::BendNorthEast),
            'J' => Ok(Self::BendNorthWest),
            '7' => Ok(Self::BendSouthWest),
            'F' => Ok(Self::BendSouthEast),
            'S' => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}

impl Pipe {
    fn edges(&self, &Xy(x, y): &Xy) -> [Xy; 2] {
        match self {
            Self::VerticalNorthSouth => [Xy(x, y - 1), Xy(x, y + 1)],
            Self::HorizontalEastWest => [Xy(x - 1, y), Xy(x + 1, y)],
            Self::BendNorthEast => [Xy(x, y - 1), Xy(x + 1, y)],
            Self::BendNorthWest => [Xy(x, y - 1), Xy(x - 1, y)],
            Self::BendSouthWest => [Xy(x, y + 1), Xy(x - 1, y)],
            Self::BendSouthEast => [Xy(x, y + 1), Xy(x + 1, y)],
            _ => unimplemented!(),
        }
    }

    fn connects(pos: &Xy, edges: &[(&Xy, &Pipe)]) -> Self {
        let candidates = edges
            .iter()
            .filter_map(|&(xy, p)| {
                let [a, b] = p.edges(xy);
                (a == *pos || b == *pos).then_some(*xy)
            })
            .collect::<HashSet<_>>();

        let interconnect = [
            Self::VerticalNorthSouth,
            Self::HorizontalEastWest,
            Self::BendNorthEast,
            Self::BendNorthWest,
            Self::BendSouthWest,
            Self::BendSouthEast,
        ]
        .iter()
        .find(|&p| {
            p.edges(pos)
                .into_iter()
                .collect::<HashSet<_>>()
                .intersection(&candidates)
                .count()
                == 2
        })
        .unwrap();

        *interconnect
    }
}

struct Map {
    pipes: HashMap<Xy, Pipe>,
    start: Xy,
}

impl From<&str> for Map {
    fn from(raw: &str) -> Self {
        let mut pipes: HashMap<_, _> = raw
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, c)| {
                    Pipe::try_from(c)
                        .map(|p| (Xy(x.try_into().unwrap(), y.try_into().unwrap()), p))
                        .ok()
                })
            })
            .collect();

        let (&start, _) = pipes
            .iter()
            .find(|(_, p)| matches!(p, Pipe::Unknown))
            .unwrap();

        let neighbours = &start.neighbouring();
        let surrounding = pipes
            .iter()
            .filter(|&(l_xy, _)| neighbours.contains(l_xy))
            .collect::<Vec<_>>();
        let connector = Pipe::connects(&start, &surrounding);

        *pipes.get_mut(&start).unwrap() = connector;

        Self { pipes, start }
    }
}

impl Map {
    fn loop_len(&self) -> usize {
        let mut len = 0;

        let mut curr_pos = self.start;
        let mut curr_pipe = *self.pipes.get(&curr_pos).unwrap();
        let mut prev_pos = *curr_pipe.edges(&curr_pos).first().unwrap();

        loop {
            let next_pos = *curr_pipe
                .edges(&curr_pos)
                .iter()
                .find(|&e| *e != prev_pos)
                .unwrap();
            let next_pipe = *self.pipes.get(&next_pos).unwrap();

            (prev_pos, curr_pos) = (curr_pos, next_pos);
            curr_pipe = next_pipe;

            len += 1;

            if curr_pos == self.start {
                return len;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT_A: &str = include_str!("../input_test_a");
    static INPUT_B: &str = include_str!("../input_test_b");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT_A), 4);
        assert_eq!(super::part1(INPUT_B), 8);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT_A), super::Answer::default());
        assert_eq!(super::part2(INPUT_B), super::Answer::default());
    }
}
