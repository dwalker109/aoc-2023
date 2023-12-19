use nom::branch::alt;
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::{alpha1, digit1, one_of};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::collections::HashMap;
use std::ops::RangeInclusive;

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
        let res = cb.0(data.clone());
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

#[derive(Debug, Clone)]
pub struct Data {
    pub x: RangeInclusive<usize>,
    pub m: RangeInclusive<usize>,
    pub a: RangeInclusive<usize>,
    pub s: RangeInclusive<usize>,
}

impl Data {
    pub fn tot(&self) -> usize {
        let Data { x, m, a, s } = self.clone();

        x.count() * m.count() * a.count() * s.count()
    }
}

pub fn parse(input: &'static str) -> HashMap<&'static str, Vec<Callback>> {
    let (rules, _) = input.split_once("\n\n").unwrap();

    fn rules_wrapper(input: &'static str) -> IResult<&'static str, (&'static str, &'static str)> {
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
                            let src = match prop {
                                'x' => d.x.clone(),
                                'm' => d.m.clone(),
                                'a' => d.a.clone(),
                                's' => d.s.clone(),
                                _ => unimplemented!(),
                            };

                            let (start, end) = (*src.start(), *src.end());
                            // let (a, b) = (*src.start()..=num - 1, num..=*src.end());

                            let (a, b) = match op {
                                '<' => (start..=num - 1, num..=end),
                                '>' => (num + 1..=end, start..=num),
                                _ => unimplemented!(),
                            };

                            let (a, b) = (match prop {
                                'x' => (Data { x: a, ..d.clone() }, Data { x: b, ..d.clone() }),
                                'm' => (Data { m: a, ..d.clone() }, Data { m: b, ..d.clone() }),
                                'a' => (Data { a: a, ..d.clone() }, Data { a: b, ..d.clone() }),
                                's' => (Data { s: a, ..d.clone() }, Data { s: b, ..d.clone() }),
                                _ => unimplemented!(),
                            });

                            (Outcome::Forward(lbl, a), Some(Outcome::Next(b)))
                        }))
                    },
                ),
                map(alpha1, |s: &'static str| {
                    Callback(Box::new(move |d| match s {
                        "A" => (Outcome::Accept(d), None),
                        "R" => (Outcome::Reject(d), None),
                        _ => (Outcome::Forward(s, d), None),
                    }))
                }),
            )),
        )(input)
    }

    let rules = rules
        .lines()
        .map(|l| {
            let (_input, (label, r)) = rules_wrapper(l).unwrap();
            let (_, rules) = raw_rules(r).unwrap();
            (label, rules)
        })
        .collect::<HashMap<_, _>>();

    rules
}
