use crate::ipc::Repo;
use crate::module::{Broadcast, Button, Communicate, Conjunction, FlipFlop, ModId};
use std::collections::HashMap;

static INPUT: &str = include_str!("../../../input/day20");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let mut repo = parse(input);

    (0..1000).for_each(|_| repo.push_the_button());

    repo.lo_qty * repo.hi_qty
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

fn parse(input: &str) -> Repo {
    let mut items = HashMap::<ModId, Box<dyn Communicate>>::new();

    items.insert("button".into(), Box::new(Button::new()));

    for l in input.lines() {
        let (id, dest) = l.split_once("->").unwrap();
        let dest = dest
            .trim()
            .split(", ")
            .map(str::to_owned)
            .collect::<Vec<ModId>>();

        match id.chars().next().unwrap() {
            'b' => {
                let id = id.trim().to_string();
                let m = Broadcast::new(id.clone(), dest);
                items.insert(id, Box::new(m));
            }
            '%' => {
                let id = id.strip_prefix('%').unwrap().trim().to_string();
                let m = FlipFlop::new(id.clone(), dest);
                items.insert(id, Box::new(m));
            }
            '&' => {
                let id = id.strip_prefix('&').unwrap().trim().to_string();

                let mem_src = input
                    .lines()
                    .filter_map(|l| {
                        let (mut mem_id, dest) = l.split_once("->").unwrap();

                        mem_id = mem_id.trim();

                        if mem_id.chars().next().unwrap().is_ascii_punctuation() {
                            mem_id = &mem_id[1..];
                        }

                        dest.trim()
                            .split(", ")
                            .any(|dest_id| dest_id == &id)
                            .then_some(mem_id.to_owned())
                    })
                    .collect::<Vec<ModId>>();

                let m = Conjunction::new(id.clone(), dest, &mem_src);
                items.insert(id, Box::new(m));
            }
            _ => unimplemented!(),
        }
    }

    Repo::new(items)
}

mod ipc;
mod module;

#[cfg(test)]
mod tests {
    static INPUT_1: &str = include_str!("../input_test_1");
    static INPUT_2: &str = include_str!("../input_test_2");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT_1), 32000000);
        assert_eq!(super::part1(INPUT_2), 11687500);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT_1), super::Answer::default());
    }
}
