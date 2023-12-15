use std::collections::{HashMap, VecDeque};

static INPUT: &str = include_str!("../../../input/day15");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    parse_1(input).map(hash).sum()
}

fn part2(input: &'static str) -> Answer {
    let mut boxes = HashMap::<usize, VecDeque<(&str, u8)>>::new();

    for op in parse_2(input) {
        op.process(&mut boxes);
    }

    boxes
        .iter()
        .map(|(box_id, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(i, (_, focal_len))| (box_id + 1) * (i + 1) * (*focal_len as usize))
                .sum::<usize>()
        })
        .sum()
}

fn parse_1(input: &str) -> impl Iterator<Item = &str> {
    input.lines().next().unwrap().split(',')
}

fn parse_2(input: &'static str) -> impl Iterator<Item = Op> {
    input.lines().next().unwrap().split(',').map(Op::from)
}

fn hash(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, curr| ((*curr as usize + acc) * 17).rem_euclid(256))
}

type Boxes = HashMap<usize, VecDeque<(&'static str, u8)>>;

enum Op {
    Remove(usize, &'static str),
    Put(usize, &'static str, u8),
}

impl From<&'static str> for Op {
    fn from(value: &'static str) -> Self {
        if value.ends_with('-') {
            let label = value.strip_suffix('-').unwrap();
            Self::Remove(hash(label), label)
        } else {
            let (label, focal_len) = value.split_once('=').unwrap();
            Self::Put(hash(label), label, focal_len.parse().unwrap())
        }
    }
}

impl Op {
    fn process(&self, boxes: &mut Boxes) {
        match self {
            Op::Remove(box_id, label) => {
                if let Some(lenses) = boxes.get_mut(box_id) {
                    lenses
                        .iter()
                        .position(|(other_label, _)| label == other_label)
                        .and_then(|pos| lenses.remove(pos));
                }
            }
            Op::Put(box_id, label, focal_len) => {
                let lenses = boxes.entry(*box_id).or_default();

                let existing_pos = lenses
                    .iter()
                    .position(|(other_label, _)| label == other_label);

                lenses.push_back((label, *focal_len));

                if let Some(pos) = existing_pos {
                    lenses.swap_remove_back(pos);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 1320);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 145);
    }
}
