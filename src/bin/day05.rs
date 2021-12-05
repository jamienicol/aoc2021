use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_point(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(
        map_res(digit1, |s: &str| s.parse()),
        tag(","),
        map_res(digit1, |s: &str| s.parse()),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, (start, end)) = separated_pair(parse_point, tag(" -> "), parse_point)(input)?;

    Ok((input, Line { start, end }))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(newline, parse_line)(input)
}

fn points_in_line(line: &Line, allow_diagonal: bool) -> Box<dyn Iterator<Item = (usize, usize)>> {
    if line.start.1 == line.end.1 {
        // Horizontal line
        let xiter = line.start.0.min(line.end.0)..=line.start.0.max(line.end.0);

        Box::new(xiter.zip(std::iter::once(line.start.1).cycle()))
    } else if line.start.0 == line.end.0 {
        // Vertical line
        let yiter = line.start.1.min(line.end.1)..=line.start.1.max(line.end.1);

        Box::new(std::iter::once(line.start.0).cycle().zip(yiter))
    } else if allow_diagonal {
        let xiter = line.start.0.min(line.end.0)..=line.start.0.max(line.end.0);
        let yiter = line.start.1.min(line.end.1)..=line.start.1.max(line.end.1);

        // If either of the x or y values is decreasing then we have to reverse
        // one of the iterators, but it doesn't matter which one.
        // If neither or both of the x or y values is decreasing then we don't.
        if (line.start.0 > line.end.0) ^ (line.start.1 > line.end.1) {
            Box::new(xiter.rev().zip(yiter))
        } else {
            Box::new(xiter.zip(yiter))
        }
    } else {
        Box::new(std::iter::empty())
    }
}

fn part_a() -> Result<usize> {
    let input = std::fs::read_to_string("res/day05")?;

    let (_, lines) = parse_lines(&input).map_err(|e| anyhow!("{:?}", e))?;

    let counts: HashMap<(usize, usize), usize> = lines
        .iter()
        .flat_map(|line| points_in_line(line, false))
        .fold(HashMap::new(), |mut counts, pos| {
            *counts.entry(pos).or_default() += 1;

            counts
        });

    let result = counts.iter().filter(|(_pos, count)| **count >= 2).count();
    Ok(result)
}

fn part_b() -> Result<usize> {
    let input = std::fs::read_to_string("res/day05")?;

    let (_, lines) = parse_lines(&input).map_err(|e| anyhow!("{:?}", e))?;

    let counts: HashMap<(usize, usize), usize> = lines
        .iter()
        .flat_map(|line| points_in_line(line, true))
        .fold(HashMap::new(), |mut counts, pos| {
            *counts.entry(pos).or_default() += 1;

            counts
        });

    let result = counts.iter().filter(|(_pos, count)| **count >= 2).count();
    Ok(result)
}

fn main() -> Result<()> {
    let result_a = part_a()?;
    println!("Day 5, part A: {}", result_a);

    let result_b = part_b()?;
    println!("Day 5, part B: {}", result_b);

    Ok(())
}
