use crate::Dir;
use geo::{coord, Area, Coord, EuclideanLength, LineString, Polygon};
use nom::bytes::complete::{take, take_until};
use nom::sequence::tuple;
use nom::IResult;

struct DigInst(Coord);

pub fn parse(input: &str) -> Polygon {
    fn extract_from_hex(l: &str) -> IResult<&str, DigInst> {
        let (rest, (_, len, dir)) = tuple((take_until("#"), take(6usize), take(1usize)))(l)?;

        let len = usize::from_str_radix(len.strip_prefix('#').unwrap(), 16).unwrap() as f64;

        let dir = match dir {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
            _ => unimplemented!(),
        };

        let transform = match dir {
            Dir::Up => coord! {x: 0.0, y: -len},
            Dir::Down => coord! {x: 0.0, y: len},
            Dir::Left => coord! {x: -len, y: 0.0},
            Dir::Right => coord! {x: len, y: 0.0},
        };

        Ok((rest, DigInst(transform)))
    }

    let ls = input
        .lines()
        .filter_map(|l| extract_from_hex(l).map(|(_, d)| d).ok())
        .fold(vec![coord! {x:0.0, y:0.0}], |mut acc, d| {
            let next = acc.last().cloned().unwrap() + d.0;
            acc.push(next);

            acc
        })
        .into_iter()
        .collect::<LineString>();

    Polygon::new(ls, Vec::default())
}

// The poly area is calculated from a series of f64 points with no area, which is quite
// different to the 1 unit wide trench we've started from. This means the area is effectively
// calculated from the mid-point of the trench, so we have to add on the outer half (edge / 2)
// plus four half unit corners. It is hard to intuit but makes sense eventually...
pub fn calc_area_with_trench(poly: &Polygon) -> usize {
    (poly.unsigned_area() + (poly.exterior().euclidean_length() / 2.0) + (0.50 * 2.0)) as usize
}
