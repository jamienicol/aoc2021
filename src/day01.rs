use std::num::ParseIntError;

use anyhow::Result;

pub fn part_a() -> Result<usize> {
    let input = std::fs::read_to_string("res/day01")?
        .lines()
        .map(|l| l.parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let result = input
        .windows(2)
        .map(|depths| (depths.first().unwrap(), depths.last().unwrap()))
        .filter(|(a, b)| b > a)
        .count();

    Ok(result)
}

pub fn part_b() -> Result<usize> {
    let input = std::fs::read_to_string("res/day01")?
        .lines()
        .map(|l| l.parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let result = input
        .windows(4)
        .map(|depths| (depths.first().unwrap(), depths.last().unwrap()))
        .filter(|(a, b)| b > a)
        .count();

    Ok(result)
}
