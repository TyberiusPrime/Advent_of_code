#![allow(unused_imports)]
use advent::prelude::*;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, line_ending, multispace0, multispace1, newline, space0},
    combinator::{map_res, opt},
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};
use std::collections::HashSet;
use std::str::FromStr;

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(" "), parse_i64)(input)
}
fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list0(newline, parse_line)(input)
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    assert!(parse_i64("75").is_ok());
    assert!(parse_line("75 75").is_ok());
    let parsed = parse_lines(input).unwrap();
    parsed.1
}

fn default_input() -> Vec<Vec<i64>> {
    parse_input(include_input!(2024 / 02))
}

fn part1(input: Vec<Vec<i64>>) -> i64 {
    let mut counter = 0;
    for report in input.iter() {
        let steps: Vec<_> = report
            .iter()
            .zip(report.iter().skip(1))
            .map(|(a, b)| (b - a))
            .collect();
        let ascending = steps.iter().all(|&x| x > 0);
        let descending = steps.iter().all(|&x| x < 0);
        let min_difference = steps.iter().map(|&x| x.abs()).min().unwrap();
        let max_difference = steps.iter().map(|&x| x.abs()).max().unwrap();
        if (ascending || descending) && min_difference >= 1 && max_difference <= 3 {
            counter += 1;
        }
    }
    counter
}

fn check_safe_part2(report: &Vec<i64>) -> bool {
    fn check_safe(report: &Vec<i64>) -> bool {
       let steps: Vec<_> = report
            .iter()
            .zip(report.iter().skip(1))
            .map(|(a, b)| (b - a))
            .collect();
        let ascending = steps.iter().all(|&x| x > 0);
        let descending = steps.iter().all(|&x| x < 0);
        let min_difference = steps.iter().map(|&x| x.abs()).min().unwrap();
        let max_difference = steps.iter().map(|&x| x.abs()).max().unwrap();
        return (ascending || descending) && min_difference >= 1 && max_difference <= 3
    }
    if check_safe(report) {
        return true;
    }
    for ii in 0..report.len() {
        let report_missing: Vec<_> = report
            .iter().enumerate().filter(|(tt, _)| *tt != ii).map(|(_, b)| *b).collect();
        if check_safe(&report_missing) {
            return true;
        }
    }
    false
}

fn part2(input: Vec<Vec<i64>>) -> i64 {
    let mut counter = 0;
    assert!(check_safe_part2(&vec![7,6,4,2,1]));
    assert!(!check_safe_part2(&vec![1,2,7,8,9]));
    assert!(!check_safe_part2(&vec![9,7,6,2,1]));
    assert!(check_safe_part2(&vec![1,3,2,4,5]));
    assert!(check_safe_part2(&vec![8,6,4,4,1]));
    assert!(check_safe_part2(&vec![1,3,6,7,9]));
    for report in input.iter() {
        if check_safe_part2(report) {
            counter += 1;
        }
        //you need to simulate leaving one out.
    }
    counter
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 321);
    assert_eq!(part2(input), 2);
}
