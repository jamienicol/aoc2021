use anyhow::Result;
use bitvec::prelude::*;

pub fn part_a() -> Result<usize> {
    let input = std::fs::read_to_string("res/day03")?;

    let values = input
        .lines()
        .map(|l| l.chars().map(|c| c == '1').collect::<BitVec<Msb0>>())
        .collect::<Vec<_>>();

    let columns = (0..values[0].len())
        .map(|i| values.iter().map(|v| v[i]).collect::<BitVec<Msb0>>())
        .collect::<Vec<_>>();

    let gamma: usize = columns
        .iter()
        .map(|c| c.count_zeros() < values.len() / 2)
        .collect::<BitVec<Msb0>>()
        .load_be();

    let epsilon: usize = columns
        .iter()
        .map(|c| c.count_zeros() >= values.len() / 2)
        .collect::<BitVec<Msb0>>()
        .load_be();

    let result = gamma * epsilon;
    Ok(result)
}

pub fn part_b() -> Result<usize> {
    let input = std::fs::read_to_string("res/day03")?;

    let values = input
        .lines()
        .map(|l| l.chars().map(|c| c == '1').collect::<BitVec<Msb0>>())
        .collect::<Vec<_>>();

    let result = 0;
    Ok(result)
}
