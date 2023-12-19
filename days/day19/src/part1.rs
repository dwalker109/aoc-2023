use nom::branch::alt;
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::{alpha1, digit1, one_of};
use nom::combinator::{map, map_res};
use nom::multi::{separated_list1};
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::collections::HashMap;

pub fn run_wf<'a, 'b>(
    wfs: &HashMap<&str, Vec<Callback>>,
    lbl: &'a str,
    data: &'b Data,
) -> Outcome<'b> {
    let wf = match lbl {
        "A" => return Outcome::Accept(data),
        "R" => return Outcome::Reject(data),
        _ => wfs.get(lbl).unwrap(),
    };

    for cb in wf.iter() {
        let res = match cb.0(data) {
            Outcome::Accept(d) => Outcome::Accept(d),
            Outcome::Reject(d) => Outcome::Reject(d),
            Outcome::Forward(lbl) => run_wf(wfs, &lbl, data),
            Outcome::Next => continue,
        };

        return res;
    }

    unreachable!();
}

pub struct Callback(Box<dyn Fn(&Data) -> Outcome>);

pub enum Outcome<'a> {
    Accept(&'a Data),
    Reject(&'a Data),
    Next,
    Forward(&'static str),
}

#[derive(Copy, Clone)]
pub struct Data {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Data {
    pub fn tot(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

pub fn parse(input: &'static str) -> (HashMap<&'static str, Vec<Callback>>, Vec<Data>) {
    let (rules, data) = input.split_once("\n\n").unwrap();

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
                        Callback(Box::new(move |d: &Data| -> Outcome {
                            let src = match prop {
                                'x' => d.x,
                                'm' => d.m,
                                'a' => d.a,
                                's' => d.s,
                                _ => unimplemented!(),
                            };

                            let res = match op {
                                '<' => (src < num)
                                    .then_some(Outcome::Forward(lbl))
                                    .or(Some(Outcome::Next)),
                                '>' => (src > num)
                                    .then_some(Outcome::Forward(lbl))
                                    .or(Some(Outcome::Next)),
                                _ => unimplemented!(),
                            };

                            res.unwrap()
                        }))
                    },
                ),
                map(alpha1, |s: &'static str| {
                    Callback(Box::new(move |d| match s {
                        "A" => Outcome::Accept(d),
                        "R" => Outcome::Reject(d),
                        _ => Outcome::Forward(s),
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

    fn data_wrapper(input: &str) -> IResult<&str, &str> {
        delimited(tag("{"), take_until1("}"), tag("}"))(input)
    }

    fn raw_data(input: &str) -> IResult<&str, Data> {
        let (input, xmas) = separated_list1(
            tag(","),
            preceded(
                tuple((one_of("xmas"), tag("="))),
                map_res(digit1, str::parse),
            ),
        )(input)?;

        let data = Data {
            x: xmas[0],
            m: xmas[1],
            a: xmas[2],
            s: xmas[3],
        };

        Ok((input, data))
    }

    let data = data
        .lines()
        .map(|l| {
            let (_input, d) = data_wrapper(l).unwrap();
            let (_, data) = raw_data(d).unwrap();
            data
        })
        .collect::<Vec<_>>();

    (rules, data)
}
