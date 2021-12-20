use std::cmp::Ordering;

use anyhow::{anyhow, Result};
use euclid::default::{Point2D, Vector2D};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct TargetArea {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

fn parse_num(input: &str) -> IResult<&str, isize> {
    map_res(
        recognize(tuple((opt(tag("-")), digit1))),
        str::parse::<isize>,
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, (isize, isize)> {
    let (input, (min, max)) = separated_pair(parse_num, tag(".."), parse_num)(input)?;

    Ok((input, (min, max)))
}

fn parse_input(input: &str) -> IResult<&str, TargetArea> {
    let (input, ((min_x, max_x), (min_y, max_y))) = separated_pair(
        preceded(tag("target area: x="), parse_range),
        tag(", y="),
        parse_range,
    )(input)?;

    Ok((
        input,
        TargetArea {
            min_x,
            max_x,
            min_y,
            max_y,
        },
    ))
}

struct Probe {
    pos: Point2D<isize>,
    velocity: Vector2D<isize>,
}

impl Probe {
    fn new(velocity: Vector2D<isize>) -> Self {
        Self {
            pos: Point2D::new(0, 0),
            velocity,
        }
    }

    fn step(&mut self) {
        self.pos += self.velocity;
        self.velocity.x -= self.velocity.x.signum();
        self.velocity.y -= 1;
    }

    fn rev_step(&mut self) {
        // self.velocity.x -= self.velocity.x.signum();
        self.velocity.y += 1;
        self.pos.y -= self.velocity.y;
    }

    fn in_target(&self, target: &TargetArea) -> bool {
        (target.min_x..=target.max_x).contains(&self.pos.x)
            && (target.min_y..=target.max_y).contains(&self.pos.y)
    }

    fn missed_target(&self, target: &TargetArea) -> bool {
        self.pos.y < target.min_y
    }
}

fn test_velocity(velocity: Vector2D<isize>, target: &TargetArea) -> Option<isize> {
    let mut probe = Probe::new(velocity);
    let mut highest = probe.pos.y;
    loop {
        probe.step();
        highest = highest.max(probe.pos.y);
        if probe.in_target(target) {
            return Some(highest);
        } else if probe.missed_target(target) {
            return None;
        }
    }
}

fn part_a(target: &TargetArea) -> Option<isize> {
    for final_y in target.min_y..=target.max_y {
        let final_vel = target.min_y - 1;
        let mut probe = Probe {
            pos: Point2D::new(0, final_y),
            velocity: Vector2D::new(0, final_vel),
        };
        let initial_vel = loop {
            probe.rev_step();

            if probe.velocity.y > 0 {
                match probe.pos.y.cmp(&0) {
                    Ordering::Equal => break Some(probe.velocity.y),
                    Ordering::Less => break None,
                    Ordering::Greater => {}
                }
            }
        };
        if let Some(initial_vel) = initial_vel {
            for x in 0..=target.max_x {
                if let Some(highest) = test_velocity(Vector2D::new(x, initial_vel), target) {
                    return Some(highest);
                }
            }
        }
    }

    None
}

fn part_b(target: &TargetArea) -> usize {
    (1..=target.max_x)
        .cartesian_product(target.min_y..(-target.min_y))
        .map(|(x, y)| Vector2D::new(x, y))
        .filter(|v| test_velocity(*v, target).is_some())
        .count()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/day17")?;
    let target = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = part_a(&target)
        .ok_or_else(|| anyhow!("Couldn't find initial velocity that lands in target"))?;
    assert_eq!(result_a, 2278);
    println!("Day 17, part A: {}", result_a);

    let result_b = part_b(&target);
    assert_eq!(result_b, 996);
    println!("Day 17, part B: {}", result_b);

    Ok(())
}
