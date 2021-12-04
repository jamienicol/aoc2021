use anyhow::Result;
use bitvec::{prelude::*};

fn parse_input() -> Result<Vec<BitVec<Msb0>>> {
    let input = std::fs::read_to_string("res/day03")?
        .lines()
        .map(|l| l.chars().map(|c| c == '1').collect())
        .collect();

    Ok(input)
}

fn transpose_values(input: &[BitVec<Msb0>]) -> Vec<BitVec<Msb0>> {
    (0..input[0].len())
        .map(|i| input.iter().map(|v| v[i]).collect::<BitVec<Msb0>>())
        .collect::<Vec<_>>()
}

pub fn part_a() -> Result<usize> {
    let values = parse_input()?;

    let columns = transpose_values(&values);

    let gamma: usize = columns
        .iter()
        .map(|c| c.count_zeros() < c.count_ones())
        .collect::<BitVec<Msb0>>()
        .load();

    let epsilon: usize = columns
        .iter()
        .map(|c| c.count_zeros() >= c.count_ones())
        .collect::<BitVec<Msb0>>()
        .load();

    let result = gamma * epsilon;
    Ok(result)
}

fn find_rating(values: Vec<BitVec<Msb0>>, most_common: bool) -> usize {
    let rating = (0..values[0].len()).fold(values.clone(), |values, i| {
        if values.len() == 1 {
            return values;
        };

        let columns = transpose_values(&values);

        values
            .into_iter()
            .filter(|v| v[i] == (columns[i].count_ones() >= columns[i].count_zeros()) ^ most_common)
            .collect::<Vec<BitVec<Msb0>>>()
    });

    assert!(rating.len() == 1);
    rating[0].load()
}

pub fn part_b() -> Result<usize> {
    let values = parse_input()?;

    let oxygen = find_rating(values.clone(), true);
    let co2 = find_rating(values, false);

    let result = oxygen * co2;
    Ok(result)
}
