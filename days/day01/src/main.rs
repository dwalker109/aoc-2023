static INPUT: &str = include_str!("../../../input/day01");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    input
        .lines()
        .map(|l| {
            let res = l.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
            [res.first().unwrap(), res.last().unwrap()]
                .into_iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn part2(input: &'static str) -> Answer {
    let lookup = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    input
        .lines()
        .map(|l| {
            let c = l.chars().collect::<Vec<_>>();
            let mut res = Vec::new();

            for n in 0..l.len() {
                let d = c[n];
                if d.is_numeric() {
                    res.push(d);
                    continue;
                }

                let x = &l[n..];
                for (s, r) in lookup.iter() {
                    if x.starts_with(s) {
                        res.push(*r);
                        continue;
                    }
                }
            }

            [
                res.first().unwrap().to_owned(),
                res.last().unwrap().to_owned(),
            ]
            .into_iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    static INPUT_1: &str = include_str!("../input_test_1");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT_1), 142);
    }

    static INPUT_2: &str = include_str!("../input_test_2");

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT_2), 281);
    }
}
