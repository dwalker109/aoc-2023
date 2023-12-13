static INPUT: &str = include_str!("../../../input/day13");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    travel(input, 0)
}

fn part2(input: &'static str) -> Answer {
    travel(input, 1)
}

fn parse(input: &str) -> impl Iterator<Item = (Vec<Vec<&u8>>, Vec<Vec<&u8>>)> {
    input.split("\n\n").map(|e| {
        let width = e.lines().next().unwrap().chars().count();
        let col_step = width + 1;
        let cols = (0..width)
            .map(|x| e.as_bytes().iter().skip(x).step_by(col_step).collect())
            .collect();

        let rows = e.lines().map(|l| l.as_bytes().iter().collect()).collect();

        (cols, rows)
    })
}

fn travel(input: &str, smudge: usize) -> Answer {
    parse(input)
        .filter_map(|(c, r)| {
            find_perfect_reflection(&c, smudge)
                .or_else(|| find_perfect_reflection(&r, smudge).map(|n| n * 100))
        })
        .sum()
}

fn find_perfect_reflection(img: &[Vec<&u8>], smudge: usize) -> Option<usize> {
    let max_n = img.len() - 1;

    for n in 1..=max_n {
        let mut smudge_remaining = smudge;

        let mut an = n - 1;
        let mut bn = n;

        loop {
            let a = &img[an];
            let b = &img[bn];

            let is_match = a.iter().zip(b.iter()).all(|(a, b)| {
                if a == b {
                    return true;
                }

                if smudge_remaining > 0 {
                    smudge_remaining -= 1;
                    return true;
                }
                return false;
            });

            match is_match {
                true => {
                    if an == 0 || bn == max_n {
                        if smudge_remaining == 0 {
                            return Some(n);
                        } else {
                            break;
                        }
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
        assert_eq!(super::part2(INPUT), 400);
    }
}
