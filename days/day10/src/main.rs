use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../../../input/day10");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let map = Map::from(input);
    map.main_loop.len() / 2
}

fn part2(input: &'static str) -> Answer {
    let map = Map::from(input);
    map.count_inner()
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
enum Tile {
    VerticalNorthSouth,
    HorizontalEastWest,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Start,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::VerticalNorthSouth,
            '-' => Self::HorizontalEastWest,
            'L' => Self::BendNorthEast,
            'J' => Self::BendNorthWest,
            '7' => Self::BendSouthWest,
            'F' => Self::BendSouthEast,
            'S' => Self::Start,
            '.' => Self::Empty,
            _ => panic!(),
        }
    }
}

impl Tile {
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

    fn connects(pos: &Xy, edges: &[(&Xy, &Tile)]) -> Self {
        let candidates = edges
            .iter()
            .filter(|(_, t)| !matches!(t, Tile::Empty))
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

    fn is_boundary_wall(&self) -> bool {
        match self {
            // Per row, only the |, F and 7 wall segments denote a cross between "inner" and "outer".
            // | is a regular wall, easy. F is opening a wall, 7 is closing it. That's all we need
            // to care about. I originally got this working with a horrible hacky and complicated
            // set of comparisons, but https://www.reddit.com/r/adventofcode/comments/18evyu9/2023_day_10_solutions
            // helped me clean it up and understand it better.
            Tile::VerticalNorthSouth | Tile::BendSouthEast | Tile::BendSouthWest => true,
            _ => false,
        }
    }
}

struct Map {
    pipes: HashMap<Xy, Tile>,
    main_loop: HashMap<Xy, Tile>,
    width_height: (usize, usize),
}

impl From<&str> for Map {
    fn from(raw: &str) -> Self {
        let width_height = (
            raw.lines().next().unwrap().chars().count(),
            raw.lines().count(),
        );

        let mut pipes: HashMap<_, _> = raw
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().map(move |(x, c)| {
                    (
                        Xy(x.try_into().unwrap(), y.try_into().unwrap()),
                        Tile::from(c),
                    )
                })
            })
            .collect();

        let (&start, _) = pipes
            .iter()
            .find(|(_, p)| matches!(p, Tile::Start))
            .unwrap();

        let neighbours = &start.neighbouring();
        let surrounding = pipes
            .iter()
            .filter(|&(l_xy, _)| neighbours.contains(l_xy))
            .collect::<Vec<_>>();
        let connector = Tile::connects(&start, &surrounding);

        *pipes.get_mut(&start).unwrap() = connector;

        let mut curr_pos = start;
        let mut curr_pipe = *pipes.get(&curr_pos).unwrap();
        let mut prev_pos = *curr_pipe.edges(&curr_pos).first().unwrap();
        let mut main_loop = HashMap::new();

        loop {
            main_loop.insert(curr_pos, curr_pipe);

            let next_pos = *curr_pipe
                .edges(&curr_pos)
                .iter()
                .find(|&e| *e != prev_pos)
                .unwrap();
            let next_pipe = *pipes.get(&next_pos).unwrap();

            (prev_pos, curr_pos) = (curr_pos, next_pos);
            curr_pipe = next_pipe;

            if curr_pos == start {
                break;
            }
        }

        Self {
            pipes,
            main_loop,
            width_height,
        }
    }
}

impl Map {
    fn count_inner(&self) -> usize {
        let mut count = 0;

        for y in 0..self.width_height.1 {
            let mut inner = false;
            for x in 0..self.width_height.0 {
                if let Some(t) = self.main_loop.get(&Xy(x as isize, y as isize)) {
                    if t.is_boundary_wall() {
                        inner = !inner;
                    }
                } else if inner {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    static INPUT_A: &str = include_str!("../input_test_a");
    static INPUT_B: &str = include_str!("../input_test_b");
    static INPUT_C: &str = include_str!("../input_test_c");
    static INPUT_D: &str = include_str!("../input_test_d");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT_A), 4);
        assert_eq!(super::part1(INPUT_B), 8);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT_C), 4);
        assert_eq!(super::part2(INPUT_D), 10);
    }
}
