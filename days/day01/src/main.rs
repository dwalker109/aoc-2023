static INPUT: &str = include_str!("../../../input/day01");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    input
        .lines()
        .map(|l| {
            let first = l.chars().find(|c| c.is_numeric()).unwrap();
            let last = l.chars().rfind(|c| c.is_numeric()).unwrap();
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum()
}

fn part2(input: &'static str) -> Answer {
    let lookup_sub = |l: &str, i: usize, b: &u8| -> Option<char> {
        let c = char::from(*b);
        if c.is_numeric() {
            return Some(c);
        }

        [
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]
        .iter()
        .find_map(|(a, b)| l.as_bytes()[i..].starts_with(a.as_bytes()).then_some(*b))
    };

    input
        .lines()
        .map(|l| {
            let first = l
                .as_bytes()
                .iter()
                .enumerate()
                .find_map(|(i, b)| lookup_sub(l, i, b))
                .unwrap();

            let last = l
                .as_bytes()
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, b)| lookup_sub(l, i, b))
                .unwrap();

            format!("{first}{last}").parse::<usize>().unwrap()
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
