

static INPUT: &str = include_str!("../../../input/day19");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    use part1::*;

    let (rules, data) = parse(input);

    data.iter()
        .filter_map(|d| match run_wf(&rules, "in", d) {
            Outcome::Accept(d) => Some(d.tot()),
            _ => None,
        })
        .sum()
}

fn part2(input: &'static str) -> Answer {
    use part2::*;

    let rules = parse(input);
    let mut accepted = Vec::new();

    run_wf(
        &rules,
        &mut accepted,
        "in",
        Data {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        },
    );

    accepted.iter().map(|d| d.tot()).sum()
}

mod part1;
mod part2;

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 19114);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 167409079868000);
    }
}
