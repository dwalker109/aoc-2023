use crate::ipc::{Queue, Repo};
use crate::module::{Broadcast, Button, Communicate, Conjunction, FlipFlop, ModId};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

static INPUT: &str = include_str!("../../../input/day20");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let (mut repo, mut queue) = parse(input);

    repo.push_the_button();

    repo.cleanup();
    queue.drain(&mut repo);
    queue.sent_product()
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

fn parse(input: &str) -> (Repo, Queue) {
    let queue = Queue::new();

    let mut items = HashMap::<ModId, Box<dyn Communicate>>::new();
    items.insert(
        "button".into(),
        Box::new(Button::new(queue.sender.clone().unwrap())),
    );

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
                let m = Broadcast::new(id.clone(), dest, queue.sender.clone().unwrap());
                items.insert(id, Box::new(m));
            }
            '%' => {
                let id = id.strip_prefix('%').unwrap().trim().to_string();
                let m = FlipFlop::new(id.clone(), dest, queue.sender.clone().unwrap());
                items.insert(id, Box::new(m));
            }
            '&' => {
                let id = id.strip_prefix('&').unwrap().trim().to_string();

                let mem_src = input
                    .lines()
                    .filter_map(|l| {
                        let (mem_id, dest) = l.split_once("->").unwrap();
                        dest.trim()
                            .split(", ")
                            .any(|dest_id| dest_id == &id)
                            .then_some(mem_id.to_owned())
                    })
                    .collect::<Vec<ModId>>();

                let m = Conjunction::new(
                    id.clone(),
                    dest,
                    queue.sender.clone().unwrap().clone(),
                    &mem_src,
                );
                items.insert(id, Box::new(m));
            }
            _ => unimplemented!(),
        }
    }

    (Repo::new(items), queue)
}

mod ipc;
mod module;

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test_1");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 36);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
