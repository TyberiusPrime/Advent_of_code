#![allow(unused_imports)]
use advent::prelude::*;
use nom::{
    bytes::complete::take_while1,
    character::complete::{digit1, multispace0, multispace1},
    combinator::{map_res, opt},
    multi::many1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

pub fn parse_lines(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    many1(preceded(
        multispace0,
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<i64>()),
            multispace1,
            map_res(digit1, |s: &str| s.parse::<i64>()),
        ),
    ))(input)
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let parsed = parse_lines(input).unwrap().1;
    let parsed: (Vec<i64>, Vec<i64>) = parsed.into_iter().unzip();
    parsed
}

fn default_input() -> (Vec<i64>, Vec<i64>) {
    parse_input(include_input!(2024 / 01))
}

fn part1(input: (Vec<i64>, Vec<i64>)) -> i64 {
    let mut a = input.0;
    let mut b = input.1;
    a.sort();
    b.sort();
    let mut delta = 0;
    for (x,y) in a.iter().zip(b.iter()) {
        delta += (x -y).abs()
    }
    return delta;
}

fn part2(input: (Vec<i64>, Vec<i64>)) -> i64 {
    let mut counts: HashMap<i64, i64> = HashMap::new();
    for ii in input.1 {
        *counts.entry(ii).or_insert(0) += 1;
    }
    let mut score = 0 as i64;
    for ii in input.0 {
        score += ii * counts.get(&ii).unwrap_or(&0);
    }
    score
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1);
    assert_eq!(part2(input), 2);
}
