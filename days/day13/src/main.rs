static INPUT: &str = include_str!("../../../input/day13");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let imgs = parse(input);

    imgs.iter()
        .filter_map(|(c, r)| {
            find_perfect_reflection(c).or_else(|| find_perfect_reflection(r).map(|n| n * 100))
        })
        .sum()
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

fn parse(input: &str) -> Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> {
    input
        .split("\n\n")
        .map(|e| {
            let width = e.lines().next().unwrap().chars().count();
            let col_step = width + 1;
            let cols = (0..width)
                .map(|x| e.chars().skip(x).step_by(col_step).collect())
                .collect();

            let rows = e.lines().map(|l| l.chars().collect()).collect();

            (cols, rows)
        })
        .collect()
}

fn find_perfect_reflection(img: &[Vec<char>]) -> Option<usize> {
    let max_n = img.len() - 1;
    for n in 1..=max_n {
        let mut an = n - 1;
        let mut bn = n;

        loop {
            let a = &img[an];
            let b = &img[bn];

            match a == b {
                true => {
                    if an == 0 || bn == max_n {
                        return Some(n);
                    }

                    an -= 1;
                    bn += 1;
                }
                false => break,
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 405);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
