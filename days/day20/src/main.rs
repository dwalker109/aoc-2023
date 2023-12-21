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
    let mut repo = parse(input);

    repo.push_the_button_find_rx()
}

fn parse(input: &'static str) -> Repo {
    let mut items = HashMap::<ModId, Box<dyn Communicate>>::new();

    items.insert("button", Box::new(Button::new()));

    for l in input.lines() {
        let (id, dest) = l.split_once("->").unwrap();
        let dest = dest.trim().split(", ").collect::<Vec<ModId>>();

        match id.chars().next().unwrap() {
            'b' => {
                let id = id.trim();
                let m = Broadcast::new(id, dest);
                items.insert(id, Box::new(m));
            }
            '%' => {
                let id = id.strip_prefix('%').unwrap().trim();
                let m = FlipFlop::new(id, dest);
                items.insert(id, Box::new(m));
            }
            '&' => {
                let id = id.strip_prefix('&').unwrap().trim();

                let mut mem_src: Vec<ModId> = Vec::new();
                for l in input.lines() {
                    let (mut mem_id, dest) = l.split_once("->").unwrap();

                    mem_id = mem_id.trim();

                    if mem_id.chars().next().unwrap().is_ascii_punctuation() {
                        mem_id = &mem_id[1..];
                    }

                    if dest.trim().split(", ").any(|dest_id| dest_id == id) {
                        mem_src.push(mem_id)
                    }
                }

                let m = Conjunction::new(id, dest, &mem_src);
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
        println!("part 2 only works on real input, no test");
    }
}
