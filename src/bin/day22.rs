use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map_res, opt, recognize, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Cuboid {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

impl Cuboid {
    fn overlaps(&self, other: &Cuboid) -> bool {
        self.x_min <= other.x_max
            && other.x_min <= self.x_max
            && self.y_min <= other.y_max
            && other.y_min <= self.y_max
            && self.z_min <= other.z_max
            && other.z_min <= self.z_max
    }

    fn contains(&self, other: &Cuboid) -> bool {
        let x_range = self.x_min..self.x_max;
        let y_range = self.y_min..self.y_max;
        let z_range = self.z_min..self.z_max;

        x_range.contains(&other.x_min)
            && x_range.contains(&other.x_max)
            && y_range.contains(&other.y_min)
            && y_range.contains(&other.y_max)
            && z_range.contains(&other.z_min)
            && z_range.contains(&other.z_max)
    }

    /// Subtract other from self and return the result as a list of cuboids
    fn sub(&self, other: &Cuboid) -> Vec<Cuboid> {
        let left = Cuboid {
            x_min: self.x_min,
            x_max: self.x_max.min(other.x_min),
            y_min: self.y_min,
            y_max: self.y_max,
            z_min: self.z_min,
            z_max: self.z_max,
        };
        let right = Cuboid {
            x_min: self.x_min.max(other.x_max),
            x_max: self.x_max,
            y_min: self.y_min,
            y_max: self.y_max,
            z_min: self.z_min,
            z_max: self.z_max,
        };
        let top = Cuboid {
            x_min: self.x_min.max(other.x_min),
            x_max: self.x_max.min(other.x_max),
            y_min: self.y_min,
            y_max: self.y_max.min(other.y_min),
            z_min: self.z_min,
            z_max: self.z_max,
        };
        let bottom = Cuboid {
            x_min: self.x_min.max(other.x_min),
            x_max: self.x_max.min(other.x_max),
            y_min: self.y_min.max(other.y_max),
            y_max: self.y_max,
            z_min: self.z_min,
            z_max: self.z_max,
        };
        let front = Cuboid {
            x_min: self.x_min.max(other.x_min),
            x_max: self.x_max.min(other.x_max),
            y_min: self.y_min.max(other.y_min),
            y_max: self.y_max.min(other.y_max),
            z_min: self.z_min,
            z_max: self.z_max.min(other.z_min),
        };
        let back = Cuboid {
            x_min: self.x_min.max(other.x_min),
            x_max: self.x_max.min(other.x_max),
            y_min: self.y_min.max(other.y_min),
            y_max: self.y_max.min(other.y_max),
            z_min: self.z_min.max(other.z_max),
            z_max: self.z_max,
        };
        [left, right, top, bottom, front, back]
            .iter()
            .filter(|c| c.x_max - c.x_min > 0 && c.y_max - c.y_min > 0 && c.z_max - c.z_min > 0)
            .cloned()
            .collect::<Vec<Cuboid>>()
    }
}

#[derive(Debug)]
struct Reactor {
    cuboids: Vec<Cuboid>,
}

impl Reactor {
    /// Sets a cuboid region either on or off
    fn set_cuboid(&mut self, cuboid: &Cuboid, state: bool) {
        let mut new_cuboid = Vec::new();

        if self
            .cuboids
            .iter()
            .any(|existing| existing.contains(cuboid))
            && state
        {
            // nothing to do
            return;
        }

        for existing_cuboid in &self.cuboids {
            if cuboid.overlaps(existing_cuboid) {
                // retain the area of the existing cuboid that doesn't overlap with the new one
                // if we are turning the new cuboid on we will do so below
                let existing = existing_cuboid.sub(cuboid);
                new_cuboid.extend(existing.into_iter());
            } else {
                // no overlap, so ensure we retain the existing cuboid
                new_cuboid.push(existing_cuboid.clone());
            }
        }
        // turn the new cuboid on if required. If we're turning off, then we've already done that
        // by subtracting from the overlapping existing cuboids
        if state {
            new_cuboid.push(cuboid.clone())
        }

        self.cuboids = new_cuboid;
    }
}

#[derive(Debug)]
struct Step {
    cuboid: Cuboid,
    state: bool,
}

fn parse_number(input: &str) -> IResult<&str, isize> {
    map_res(
        recognize(tuple((opt(tag("-")), digit1))),
        str::parse::<isize>,
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, (isize, isize)> {
    let (input, range) = separated_pair(parse_number, tag(".."), parse_number)(input)?;
    Ok((input, range))
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    let (input, state) = alt((value(true, tag("on")), value(false, tag("off"))))(input)?;

    let (input, ((x_min, x_max), (y_min, y_max), (z_min, z_max))) = tuple((
        preceded(tag(" x="), parse_range),
        preceded(tag(",y="), parse_range),
        preceded(tag(",z="), parse_range),
    ))(input)?;

    Ok((
        input,
        Step {
            cuboid: Cuboid {
                x_min,
                x_max: x_max + 1,
                y_min,
                y_max: y_max + 1,
                z_min,
                z_max: z_max + 1,
            },
            state,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(newline, parse_step)(input)
}

fn part_a(steps: &[Step]) -> usize {
    let mut cuboids = HashMap::new();

    for step in steps {
        for pos in (step.cuboid.x_min.max(-50)..step.cuboid.x_max.min(50)).flat_map(move |x| {
            (step.cuboid.y_min.max(-50)..step.cuboid.y_max.min(50)).flat_map(move |y| {
                (step.cuboid.z_min.max(-50)..step.cuboid.z_max.min(50)).map(move |z| (x, y, z))
            })
        }) {
            cuboids.insert(pos, step.state);
        }
    }

    cuboids.values().filter(|state| **state).count()
}

fn part_b(steps: &[Step]) -> u64 {
    let mut reactor = Reactor {
        cuboids: Vec::new(),
    };

    for step in steps {
        reactor.set_cuboid(&step.cuboid, step.state);
    }

    reactor
        .cuboids
        .iter()
        .map(|c| {
            (c.x_max - c.x_min) as u64 * (c.y_max - c.y_min) as u64 * (c.z_max - c.z_min) as u64
        })
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/day22")?;
    let steps = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = part_a(&steps);
    assert_eq!(result_a, 583636);
    println!("Day 22, part A: {}", result_a);

    let result_b = part_b(&steps);
    assert_eq!(result_b, 1294137045134837);
    println!("Day 22, part B: {}", result_b);

    Ok(())
}
