use std::collections::HashSet;

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug)]
struct HeightMap {
    heights: Vec<u32>,
    width: usize,
    length: usize,
}

impl HeightMap {
    fn get_height(&self, pos: (usize, usize)) -> u32 {
        self.heights[pos.1 * self.width + pos.0]
    }

    fn adjacent_positions(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        ((pos.0.max(1) - 1)..=(pos.0 + 1).min(self.width - 1))
            .cartesian_product((pos.1.max(1) - 1)..=(pos.1 + 1).min(self.length - 1))
            .filter(move |neighbour| {
                pos.0.max(neighbour.0) - pos.0.min(neighbour.0) + pos.1.max(neighbour.1)
                    - pos.1.min(neighbour.1)
                    == 1
            })
    }

    fn adjacent_heights(&self, pos: (usize, usize)) -> impl Iterator<Item = u32> + '_ {
        self.adjacent_positions(pos)
            .map(move |neighbour| self.get_height(neighbour))
    }

    fn is_low_point(&self, pos: (usize, usize)) -> bool {
        let height = self.get_height(pos);
        self.adjacent_heights(pos).all(|h| h > height)
    }

    fn find_basin(&self, low_point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut basin = vec![];
        let mut to_search = vec![low_point];
        let mut searched: HashSet<(usize, usize)> = HashSet::default();

        while let Some(pos) = to_search.pop() {
            searched.insert(pos);
            basin.push(pos);
            for neighbour in self
                .adjacent_positions(pos)
                .filter(|neighbour| !searched.contains(neighbour))
            {
                if self.get_height(neighbour) > self.get_height(pos)
                    && self.get_height(neighbour) != 9
                    && !to_search.contains(&neighbour)
                {
                    to_search.push(neighbour)
                }
            }
        }

        basin
    }
}

fn parse_input(input: &str) -> Result<HeightMap> {
    let width = input
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Empty input"))?
        .chars()
        .count();
    let length = input.lines().count();
    let heights = input
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| anyhow!("{} is not a valid digit", c))
        })
        .collect::<Result<Vec<u32>>>()?;

    Ok(HeightMap {
        width,
        length,
        heights,
    })
}

fn part_a(heights: &HeightMap) -> u32 {
    (0..heights.width)
        .cartesian_product(0..heights.length)
        .filter_map(|pos| {
            heights
                .is_low_point(pos)
                .then(|| heights.get_height(pos) + 1)
        })
        .sum::<u32>()
}

fn part_b(heights: &HeightMap) -> u32 {
    let low_points = (0..heights.width)
        .cartesian_product(0..heights.length)
        .filter(|(x, y)| {
            let height = heights.get_height((*x, *y));
            heights.adjacent_heights((*x, *y)).all(|h| h > height)
        });

    low_points
        .map(|low_point| heights.find_basin(low_point).len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>() as _
}

fn main() -> Result<()> {
    let input = &std::fs::read_to_string("res/day09")?;
    let heights = parse_input(input)?;

    let result_a = part_a(&heights);
    assert_eq!(result_a, 575);
    println!("Day 9, part A: {}", result_a);

    let result_b = part_b(&heights);
    assert_eq!(result_b, 1019700);
    println!("Day 9, part B: {}", result_b);

    for p in (0..=2).cartesian_product(1..=5) {
        println!("{:?}", p)
    }

    Ok(())
}
