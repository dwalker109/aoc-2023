#![feature(cursor_remaining)]

use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::format;
use std::io::{BufRead, Cursor, Read, Seek};

static INPUT: &str = include_str!("../../../input/day12");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let sources = parse(input);
    calc(&sources)
}

fn part2(input: &'static str) -> Answer {
    todo!()
}

fn calc(sources: &[(Cursor<Vec<u8>>, Vec<usize>)]) -> Answer {
    sources
        .par_iter()
        .enumerate()
        .map(|(i, (s, p))| {
            let exp = p.iter().map(|n| format!("#{{{n}}}")).join(r"\.+");
            let f = format!(r"^\.*{exp}\.*$");
            let rx = regex::Regex::new(&f).unwrap();

            let expanded = expand(s.clone());
            expanded.iter().filter(|&s| rx.is_match(s)).count()
        })
        .sum()
}

fn parse(input: &str) -> Vec<(Cursor<Vec<u8>>, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let (p, n) = l.split_once(' ').unwrap();

            let p = Cursor::new(Vec::from(p));

            let n = n
                .split(',')
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>();

            (p, n)
        })
        .collect()
}

fn expand(mut s: Cursor<Vec<u8>>) -> Vec<String> {
    let mut buf = Vec::with_capacity(s.remaining_slice().len());
    let n = s.read_until(b'?', &mut buf).unwrap();

    if n > 0 && buf.ends_with(b"?") {
        let pos = s.position() as usize - 1;

        let mut a = s.clone();
        let mut b = s;

        a.get_mut()[pos] = b'.';
        b.get_mut()[pos] = b'#';

        return [expand(a), expand(b)].into_iter().flatten().collect();
    }

    s.rewind().ok();

    vec![String::from_utf8(s.into_inner()).unwrap()]
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 21);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 525152);
    }
}
