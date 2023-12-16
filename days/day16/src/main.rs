#![feature(let_chains)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Range, RangeInclusive};

static INPUT: &str = include_str!("../../../input/day16");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let mut g = parse(input);
    g.simulate();
    g.energised.len()
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

fn parse(input: &str) -> Grid {
    Grid::from(input)
}

struct Grid {
    tiles: HashMap<Xy, Tile>,
    beams: VecDeque<Beam>,
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
    energised: HashSet<Xy>,
    loop_detect: HashSet<(Xy, Dir)>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let tiles = value
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| (Xy(x, y), Tile::from(c)))
            })
            .collect::<HashMap<_, _>>();

        let x_range = 0..=(tiles.iter().max_by_key(|(pos, _)| pos.0).unwrap().0 .0);
        let y_range = 0..=(tiles.iter().max_by_key(|(pos, _)| pos.1).unwrap().0 .1);

        let beams = VecDeque::from(vec![Beam {
            dir: Dir::Right,
            pos: Xy(0, 0),
        }]);

        let energised = HashSet::with_capacity(tiles.len());

        Self {
            tiles,
            beams,
            x_range,
            y_range,
            energised,
            loop_detect: HashSet::new(),
        }
    }
}

impl Grid {
    pub fn simulate(&mut self) {
        while let Some(mut b) = self.beams.pop_front() {
            loop {
                if !self.loop_detect.insert((b.pos, b.dir)) {
                    break;
                } else {
                    self.energised.insert(b.pos);
                }

                let t = self.tiles.get(&b.pos).unwrap();

                match t.interact(&b.dir) {
                    (next_dir, None) => {
                        if let Some(next_xy) = b.next_xy(&next_dir, &self.x_range, &self.y_range) {
                            b.dir = next_dir;
                            b.pos = next_xy;
                        } else {
                            break;
                        }
                    }
                    (next_dir_a, Some(next_dir_b)) => {
                        if let Some(next_xy) = b.next_xy(&next_dir_a, &self.x_range, &self.y_range)
                        {
                            self.beams.push_back(Beam {
                                dir: next_dir_a,
                                pos: next_xy,
                            });
                        }
                        if let Some(next_xy) = b.next_xy(&next_dir_b, &self.x_range, &self.y_range)
                        {
                            self.beams.push_back(Beam {
                                dir: next_dir_b,
                                pos: next_xy,
                            });
                        }
                        break;
                    }
                }
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Xy(usize, usize);

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

enum Tile {
    Space,
    MirrorRight,
    MirrorLeft,
    SplitterVertical,
    SplitterHorizontal,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Space,
            '/' => Self::MirrorRight,
            '\\' => Self::MirrorLeft,
            '|' => Self::SplitterVertical,
            '-' => Self::SplitterHorizontal,
            _ => unimplemented!(),
        }
    }
}

impl Tile {
    pub fn interact(&self, beam_dir: &Dir) -> (Dir, Option<Dir>) {
        match self {
            Tile::Space => (*beam_dir, None),
            Tile::MirrorRight => match beam_dir {
                Dir::Up => (Dir::Right, None),
                Dir::Down => (Dir::Left, None),
                Dir::Left => (Dir::Down, None),
                Dir::Right => (Dir::Up, None),
            },
            Tile::MirrorLeft => match beam_dir {
                Dir::Up => (Dir::Left, None),
                Dir::Down => (Dir::Right, None),
                Dir::Left => (Dir::Up, None),
                Dir::Right => (Dir::Down, None),
            },
            Tile::SplitterVertical => match beam_dir {
                Dir::Up | Dir::Down => (*beam_dir, None),
                Dir::Left => (Dir::Up, Some(Dir::Down)),
                Dir::Right => (Dir::Down, Some(Dir::Up)),
            },
            Tile::SplitterHorizontal => match beam_dir {
                Dir::Left | Dir::Right => (*beam_dir, None),
                Dir::Up => (Dir::Left, Some(Dir::Right)),
                Dir::Down => (Dir::Right, Some(Dir::Left)),
            },
        }
    }
}

struct Beam {
    dir: Dir,
    pos: Xy,
}

impl Beam {
    pub fn next_xy(
        &self,
        dir: &Dir,
        x_range: &RangeInclusive<usize>,
        y_range: &RangeInclusive<usize>,
    ) -> Option<Xy> {
        let Xy(curr_x, curr_y) = self.pos;

        match dir {
            Dir::Up => curr_y.checked_sub(1).map(|y| Xy(curr_x, y)),
            Dir::Down => curr_y
                .checked_add(1)
                .and_then(|y| y_range.contains(&y).then_some(y))
                .map(|y| Xy(curr_x, y)),
            Dir::Left => curr_x.checked_sub(1).map(|x| Xy(x, curr_y)),
            Dir::Right => curr_x
                .checked_add(1)
                .and_then(|x| x_range.contains(&x).then_some(x))
                .map(|x| Xy(x, curr_y)),
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 46);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
