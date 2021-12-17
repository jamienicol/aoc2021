use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    cells: Vec<usize>,
}

impl Map {
    fn cost(&self, pos: (usize, usize)) -> usize {
        assert!(pos.0 < self.width);
        assert!(pos.1 < self.height);

        self.cells[pos.1 * self.width + pos.0]
    }

    fn neighbours(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        ((pos.0.max(1) - 1)..=(pos.0 + 1).min(self.width - 1))
            .cartesian_product((pos.1.max(1) - 1)..=(pos.1 + 1).min(self.height - 1))
            .filter(move |neighbour| {
                pos.0.max(neighbour.0) - pos.0.min(neighbour.0) + pos.1.max(neighbour.1)
                    - pos.1.min(neighbour.1)
                    == 1
            })
    }
}

fn parse_map(input: &str) -> Result<Map> {
    let width = input
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Empty input"))?
        .chars()
        .count();
    let height = input.lines().count();

    let cells = input
        .lines()
        .map(|line| {
            if line.chars().count() != width {
                return Err(anyhow!("Line {} has unexpected length", &line));
            }

            Ok(line.chars())
        })
        .flatten_ok()
        .map(|c| match c {
            Ok(c) => Ok(c
                .to_digit(10)
                .ok_or_else(|| anyhow!("Invalid char {}", c))? as usize),
            Err(e) => Err(e),
        })
        .try_collect()?;

    Ok(Map {
        width,
        height,
        cells,
    })
}

fn generate_big_map(wee_map: &Map) -> Map {
    let width = wee_map.width * 5;
    let height = wee_map.height * 5;

    let cells = (0..height)
        .flat_map(move |y| {
            (0..5).flat_map(move |i| {
                wee_map
                    .cells
                    .iter()
                    .skip((y % wee_map.height) * wee_map.width)
                    .take(wee_map.width)
                    .map(move |c| {
                        let val = c + i + y / wee_map.height;
                        val % 10 + val / 10
                    })
            })
        })
        .collect::<Vec<usize>>();

    Map {
        width,
        height,
        cells,
    }
}

fn a_star(start: (usize, usize), end: (usize, usize), map: &Map) -> Option<usize> {
    #[derive(Debug, Clone, Copy)]
    struct Cost {
        g: usize,
        h: usize,
    }

    /// Heuristic for remaining cost from position to end.
    /// This must be "admissable", meaning it cannot overestimate the cost.
    /// We therefore just use the manhattan distance (as if every cell had a cost of 1).
    fn h(pos: (usize, usize), end: (usize, usize)) -> usize {
        pos.0.max(end.0) - pos.0.min(end.0) + pos.1.max(end.1) - pos.1.min(end.1)
    }

    let mut open: HashMap<(usize, usize), Cost> = HashMap::default();
    let mut closed: HashSet<(usize, usize)> = HashSet::default();
    open.insert(
        (0, 0),
        Cost {
            g: 0,
            h: h(start, end),
        },
    );

    while let Some((current_pos, current_cost)) = open
        .iter()
        .min_by_key(|(_pos, cost)| cost.g + cost.h)
        .map(|(pos, cost)| (*pos, *cost))
    {
        open.remove(&current_pos);
        closed.insert(current_pos);

        if current_pos == end {
            assert_eq!(current_cost.h, 0);
            return Some(current_cost.g);
        }

        // Calculate the cost for each neighbouring cell and add to open list.
        for neighbour in map
            .neighbours(current_pos)
            .filter(|neighbour| !closed.contains(neighbour))
        {
            let g = current_cost.g + map.cost(neighbour);
            let h = h(neighbour, end);
            open.entry(neighbour)
                .and_modify(|existing| {
                    assert_eq!(h, existing.h);
                    // If we've found a shorter route to an already discovered cell, update its cost.
                    existing.g = g.min(existing.g);
                })
                .or_insert(Cost { g, h });
        }
    }

    None
}

fn main() -> Result<()> {
    let input = &std::fs::read_to_string("res/day15")?;
    let map = parse_map(input)?;

    let result_a = a_star((0, 0), (map.width - 1, map.height - 1), &map)
        .ok_or_else(|| anyhow!("Failed to find path"))?;
    assert_eq!(result_a, 602);
    println!("Day 15, part A: {}", result_a);

    let big_map = generate_big_map(&map);
    let result_b = a_star((0, 0), (big_map.width - 1, big_map.height - 1), &big_map)
        .ok_or_else(|| anyhow!("Failed to find path"))?;
    assert_eq!(result_b, 2935);
    println!("Day 15, part B: {}", result_b);

    Ok(())
}
