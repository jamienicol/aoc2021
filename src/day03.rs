use std::ops::Range;

use anyhow::Result;
use bitvec::prelude::*;

fn parse_input() -> Result<Vec<BitVec<Msb0>>> {
    let input = std::fs::read_to_string("res/day03")?
        .lines()
        .map(|l| l.chars().map(|c| c == '1').collect())
        .collect();

    Ok(input)
}

// Transpose a vector of rows into a vector of the specified columns
fn transpose_values(input: &[BitVec<Msb0>], columns: Range<usize>) -> Vec<BitVec<Msb0>> {
    columns
        .map(|i| input.iter().map(|v| v[i]).collect::<BitVec<Msb0>>())
        .collect::<Vec<_>>()
}

// Returns true if there are more (or equal) 1s than 0s in a column
fn find_gamma_bit(column: &BitVec<Msb0>) -> bool {
    column.count_ones() >= column.count_zeros()
}

pub fn part_a() -> Result<usize> {
    let values = parse_input()?;

    let columns = transpose_values(&values, 0..values[0].len());

    let gamma: usize = columns
        .iter()
        .map(find_gamma_bit)
        .collect::<BitVec<Msb0>>()
        .load();

    // Rather than calculating epsilon manually we can just invert each of gamma's bits
    let epsilon = gamma ^ ((1 << values[0].len()) - 1);

    let result = gamma * epsilon;
    Ok(result)
}

// Find the oxygen or co2 rating. most_common == true indicates we are looking for values
// with the most common bit in each column, ie the oxygen rating.
fn find_rating(values: Vec<BitVec<Msb0>>, most_common: bool) -> usize {
    // Iterate through each column, starting with every row as our initial accumulator
    let rating = (0..values[0].len()).fold(values, |values, i| {
        if values.len() == 1 {
            return values;
        };

        // We only need to transpose the current column
        let column = transpose_values(&values, i..i + 1);

        let filter = find_gamma_bit(&column[0]) ^ most_common;

        // Filter the values by whether they match the most/least common bit,
        // using the output as the accumulator for the next iteration
        values
            .into_iter()
            .filter(|v| v[i] == filter)
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
