use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), map_res(digit1, str::parse))(input)
}

fn part_a(positions: &[usize]) -> Result<usize> {
    let median = positions
        .iter()
        .sorted()
        .nth(positions.len() / 2)
        .ok_or_else(|| anyhow!("Empty input"))?;

    let result = positions
        .iter()
        .fold(0, |acc, p| acc + (p.max(median) - p.min(median)));

    Ok(result)
}

fn part_b(positions: &[usize]) -> Result<usize> {
    let count = if positions.is_empty() {
        return Err(anyhow!("Empty input"));
    } else {
        positions.len()
    };

    let mean = positions.iter().map(|p| *p as f64).sum::<f64>() / count as f64;

    let find_cost = |positions: &[usize], mean| {
        positions.iter().fold(0, |acc, p| {
            let distance = p.max(&mean) - p.min(&mean);
            let cost = (1..=distance).sum::<usize>();
            acc + cost
        })
    };

    // The example input results in a mean of 4.9. Rounding this to the nearest
    // integer (5) gives the correct answer. My puzzle input, however, results
    // in a mean of 447.5. Rounding either up or down gives a different result.
    // Not sure why, but do both and pick the smallest as the answer.
    let result_floor = find_cost(positions, mean.floor() as usize);
    let result_ceil = find_cost(positions, mean.ceil() as usize);

    Ok(result_floor.min(result_ceil))
}

fn main() -> Result<()> {
    let input = &std::fs::read_to_string("res/day07")?;

    let positions = parse_input(input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = part_a(&positions)?;
    assert_eq!(result_a, 326132);
    println!("Day 7, part A: {}", result_a);

    let result_b = part_b(&positions)?;
    assert_eq!(result_b, 88612508);
    println!("Day 7, part B: {}", result_b);

    Ok(())
}
