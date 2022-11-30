use anyhow::{anyhow, Result};
use euclid::{default::Point2D, point2};

#[derive(Clone, Debug)]
enum Cucumber {
    East,
    South,
}

#[derive(Debug, Clone, PartialEq)]
struct World {
    width: usize,
    height: usize,
    east_cucumbers: Vec<Point2D<usize>>,
    south_cucumbers: Vec<Point2D<usize>>,
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        World {
            width,
            height,
            east_cucumbers: Vec::default(),
            south_cucumbers: Vec::default(),
        }
    }

    fn create_map(&self) -> Vec<Option<Cucumber>> {
        let mut map = vec![None; self.width * self.height];

        for pos in &self.east_cucumbers {
            map[pos.y * self.width + pos.x] = Some(Cucumber::East);
        }
        for pos in &self.south_cucumbers {
            map[pos.y * self.width + pos.x] = Some(Cucumber::South);
        }

        map
    }

    #[allow(dead_code)]
    fn print_map(&self) {
        let map = self.create_map();

        let mut map_str = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match map[y * self.width + x] {
                    Some(Cucumber::East) => '>',
                    Some(Cucumber::South) => 'v',
                    None => '.',
                };
                map_str.push(c);
            }
            map_str.push('\n');
        }

        println!("{}", &map_str);
    }

    fn step(&mut self) -> bool {
        let mut modified = false;

        let map = self.create_map();
        for c in &mut self.east_cucumbers {
            let new_pos = point2((c.x + 1) % self.width, c.y);
            if map[new_pos.y * self.width + new_pos.x].is_none() {
                *c = new_pos;
                modified = true;
            }
        }

        let map = self.create_map();
        for c in &mut self.south_cucumbers {
            let new_pos = point2(c.x, (c.y + 1) % self.height);
            if map[new_pos.y * self.width + new_pos.x].is_none() {
                *c = new_pos;
                modified = true;
            }
        }

        modified
    }
}

fn parse_input(input: &str) -> Result<World> {
    let width = input
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Empty input"))?
        .chars()
        .count();
    let height = input.lines().count();

    let world = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .fold(World::new(width, height), |mut world, (x, y, c)| {
            match c {
                '>' => world.east_cucumbers.push(point2(x, y)),
                'v' => world.south_cucumbers.push(point2(x, y)),
                _ => {}
            }
            world
        });

    Ok(world)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/day25")?;
    let mut world = parse_input(&input)?;

    let mut num_steps = 0;
    loop {
        let mut new_world = world.clone();
        new_world.step();
        num_steps += 1;

        if new_world == world {
            break;
        }

        world = new_world;
    }

    println!("Day 25, part A: {}", num_steps);

    Ok(())
}
