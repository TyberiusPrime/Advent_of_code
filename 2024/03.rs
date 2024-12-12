#![allow(unused_imports)]
use advent::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while, take_while1},
    character::complete::{char, digit1, line_ending, multispace0, multispace1, newline, space0},
    combinator::{map, map_res, opt},
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};
use std::collections::HashSet;
use std::str::FromStr;

/// Parses a single number, which may be positive or negative.

fn parse_number(input: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(input)
}

/// Parses the pattern `mul(number,number)`.
fn parse_valid_mul(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, _) = tag("mul(")(input)?;
    let (input, (num1, num2)) =
        pair(parse_number, delimited(char(','), parse_number, char(')')))(input)?;
    Ok((input, (num1, num2)))
}

/// Skips over malformed `mul(`, consuming up to the next occurrence of `mul(` or until input ends.
fn skip_malformed_mul(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((tag("do("), tag("don't("), tag("mul(")))(input)?;
    let (input, _) = take_while(|c| c != 'm')(input)?; // Consume until next possible "mul("
    Ok((input, ()))
}

#[derive(Clone)]
enum Instruction {
    Do,
    Dont,
    Mul(i64, i64),
}

fn parse_do(input: &str) -> IResult<&str, ()> {
    tag("do()")(input).map(|(input, _)| (input, ()))
}

fn parse_dont(input: &str) -> IResult<&str, ()> {
    tag("don't()")(input).map(|(input, _)| (input, ()))
}
/// Parses either a valid `mul(number,number)` or skips a malformed `mul(`.
fn parse_any_mul(input: &str) -> IResult<&str, Option<Instruction>> {
    alt((
        map(parse_do, |_| Some(Instruction::Do)),
        map(parse_dont, |_| Some(Instruction::Dont)),
        map(parse_valid_mul, |x| Some(Instruction::Mul(x.0, x.1))),
        map(skip_malformed_mul, |_| None),
    ))(input)
}

/// Finds all valid `mul(number,number)` patterns in the input, ignoring malformed ones.
fn find_all_mul(input: &str) -> Vec<Instruction> {
    let mut parser = many1(delimited(
        take_until(alt(("mul(", "do(", "don't("))),
        parse_any_mul,
        opt(take_until("mul(")),
    ));

    match parser(input) {
        Ok((_, results)) => results.into_iter().flatten().collect(),
        Err(_) => vec![], // Return empty vector if no matches are found
    }
}

fn default_input() -> Vec<Instruction> {
    find_all_mul(include_input!(2024 / 03))
}

fn part1(input: Vec<Instruction>) -> i64 {
    let should: Vec<(i64, i64)> =
        find_all_mul("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
            .into_iter()
            .filter_map(|x| match x {
                Instruction::Mul(a, b) => Some((a, b)),
                _ => None,
            })
            .collect();
    assert_eq!(should, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    input
        .iter()
        .filter_map(|x| match x {
            Instruction::Do => None,
            Instruction::Dont => None,
            Instruction::Mul(a, b) => Some((a, b)),
        })
        .map(|(a, b)| a * b)
        .sum()
}

fn part2(input: Vec<Instruction>) -> i64 {
    let mut out = Vec::new();
    let mut inside_do = true;
    for instruction in input {
        match instruction {
            Instruction::Do => {
                inside_do = true;
                panic!("shu")
            }
            Instruction::Dont => {
                inside_do = false;
                panic!("sha")
            }
            Instruction::Mul(a, b) => {
                if inside_do {
                    out.push((a, b))
                }
            }
        }
    }
    out.iter().map(|(a, b)| a * b).sum()
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
