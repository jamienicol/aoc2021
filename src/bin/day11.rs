use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct OctopusGrid {
    width: usize,
    height: usize,
    octopuses: Vec<u8>,
}

impl OctopusGrid {
    fn new(width: usize, height: usize, octopuses: Vec<u8>) -> Result<Self> {
        if octopuses.len() != width * height {
            return Err(anyhow!(
                "Expected {} octopuses, not {}",
                width * height,
                octopuses.len()
            ));
        }

        Ok(Self {
            width,
            height,
            octopuses,
        })
    }

    fn step(&mut self) -> usize {
        let mut to_flash = Vec::new();

        for (x, y) in (0..self.width).cartesian_product(0..self.height) {
            if self.octopuses[y * self.width + x] < 11 {
                self.octopuses[y * self.width + x] += 1;
            }

            if self.octopuses[y * self.width + x] == 10 {
                to_flash.push((x, y));
            }
        }

        while !to_flash.is_empty() {
            to_flash = to_flash
                .drain(..)
                .fold(Vec::new(), |mut acc, (flash_x, flash_y)| {
                    for (x, y) in ((flash_x.max(1) - 1)..=(flash_x + 1).min(self.width - 1))
                        .cartesian_product(
                            (flash_y.max(1) - 1)..=(flash_y + 1).min(self.height - 1),
                        )
                        .filter(|(x, y)| *x != flash_x || *y != flash_y)
                    {
                        if self.octopuses[y * self.width + x] < 11 {
                            self.octopuses[y * self.width + x] += 1;
                        }

                        if self.octopuses[y * self.width + x] == 10 {
                            acc.push((x, y));
                        }
                    }
                    acc
                });
        }

        let mut num_flashes = 0;
        for (x, y) in (0..self.width).cartesian_product(0..self.height) {
            if self.octopuses[y * self.width + x] > 9 {
                num_flashes += 1;
                self.octopuses[y * self.width + x] = 0;
            }
        }

        num_flashes
    }
}

fn part_a(mut grid: OctopusGrid) -> usize {
    (0..100).map(|_| grid.step()).sum::<usize>()
}

fn part_b(mut grid: OctopusGrid) -> usize {
    let mut n = 0;
    loop {
        n += 1;
        if grid.step() == grid.octopuses.len() {
            return n;
        }
    }
}

fn main() -> Result<()> {
    let input = &std::fs::read_to_string("res/day11")?;

    let octopuses = input
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| anyhow!("{} is not a valid digit", c))
                .map(|d| d as u8)
        })
        .collect::<Result<Vec<u8>>>()?;

    let grid = OctopusGrid::new(10, 10, octopuses)?;

    let result_a = part_a(grid.clone());
    assert_eq!(result_a, 1721);
    println!("Day 11, part A: {}", result_a);

    let result_b = part_b(grid);
    assert_eq!(result_b, 298);
    println!("Day 11, part B: {}", result_b);

    Ok(())
}
