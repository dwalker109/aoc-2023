use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{alpha1, digit1, one_of},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
use std::collections::HashMap;

pub fn run_wf(
    wfs: &HashMap<&str, Vec<Callback>>,
    accepted: &mut Vec<Data>,
    lbl: &str,
    mut data: Data,
) {
    let wf = match lbl {
        "A" => {
            return accepted.push(data);
        }
        "R" => return,
        _ => wfs.get(lbl).unwrap(),
    };

    for cb in wf.iter() {
        let res = cb.0(data);
        match res {
            (Outcome::Accept(ad), None) => {
                accepted.push(ad);
            }
            (Outcome::Forward(fl, fd), None) => {
                run_wf(wfs, accepted, fl, fd);
            }
            (Outcome::Forward(fl, fd), Some(Outcome::Next(nd))) => {
                run_wf(wfs, accepted, fl, fd);
                data = nd; // Will be picked up next time
            }
            _ => (),
        };
    }
}

pub struct Callback(Box<dyn Fn(Data) -> (Outcome, Option<Outcome>)>);

#[derive(Debug)]
pub enum Outcome {
    Accept(Data),
    Reject(Data),
    Next(Data),
    Forward(&'static str, Data),
}

#[derive(Debug, Copy, Clone)]
pub struct Data([(usize, usize); 4]);

impl Data {
    pub fn new() -> Self {
        Self([(1, 4000), (1, 4000), (1, 4000), (1, 4000)])
    }

    fn idx(&self, key: char) -> usize {
        match key {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unimplemented!(),
        }
    }
    fn get(&self, key: char) -> &(usize, usize) {
        &self.0[self.idx(key)]
    }

    fn copied(&self, key: char, val: (usize, usize)) -> Self {
        let mut new = *self;
        new.0[self.idx(key)] = val;

        new
    }

    pub fn tot(&self) -> usize {
        self.0.iter().map(|(a, b)| a.abs_diff(*b) + 1).product()
    }
}

pub fn parse(input: &'static str) -> HashMap<&'static str, Vec<Callback>> {
    let (rules, _) = input.split_once("\n\n").unwrap();

    fn rules_wrapper(input: &str) -> IResult<&str, (&str, &str)> {
        tuple((alpha1, delimited(tag("{"), take_until1("}"), tag("}"))))(input)
    }

    fn raw_rules(input: &'static str) -> IResult<&'static str, Vec<Callback>> {
        separated_list1(
            tag(","),
            alt((
                map(
                    tuple((
                        one_of("xmas"),
                        one_of("<>"),
                        map_res(digit1, str::parse),
                        tag(":"),
                        alpha1,
                    )),
                    |(prop, op, num, _, lbl): (char, char, usize, &str, &str)| {
                        Callback(Box::new(move |d: Data| {
                            let (start, end) = *d.get(prop);

                            let (a, b) = match op {
                                '<' => ((start, num - 1), (num, end)),
                                '>' => ((num + 1, end), (start, num)),
                                _ => unimplemented!(),
                            };

                            let (a, b) = (d.copied(prop, a), d.copied(prop, b));

                            (Outcome::Forward(lbl, a), Some(Outcome::Next(b)))
                        }))
                    },
                ),
                map(alpha1, |s: &str| {
                    Callback(Box::new(move |d| match s {
                        "A" => (Outcome::Accept(d), None),
                        "R" => (Outcome::Reject(d), None),
                        _ => (Outcome::Forward(s, d), None),
                    }))
                }),
            )),
        )(input)
    }

    rules
        .lines()
        .map(|l| {
            let (_input, (label, r)) = rules_wrapper(l).unwrap();
            let (_, rules) = raw_rules(r).unwrap();
            (label, rules)
        })
        .collect::<HashMap<_, _>>()
}
