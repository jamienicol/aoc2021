use std::num::ParseIntError;

use anyhow::Result;

/// Counts the number of increases of average depth between
/// consecutive windows over a specified size.
fn count_increases(depths: &[usize], window_size: usize) -> usize {
    depths
        .iter()
        .zip(depths.iter().skip(window_size))
        .filter(|(a, b)| b > a)
        .count()
}

fn part_a() -> Result<usize> {
    let input = std::fs::read_to_string("res/day01")?
        .lines()
        .map(|l| l.parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let result = count_increases(&input, 1);

    Ok(result)
}

fn part_b() -> Result<usize> {
    let input = std::fs::read_to_string("res/day01")?
        .lines()
        .map(|l| l.parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let result = count_increases(&input, 3);

    Ok(result)
}

fn main() -> Result<()> {
    let result_a = part_a()?;
    println!("Day 1, part A: {}", result_a);

    let result_b = part_b()?;
    println!("Day 1, part B: {}", result_b);

    Ok(())
}
