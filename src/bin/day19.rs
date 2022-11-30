use std::collections::{HashSet, VecDeque};

use anyhow::{anyhow, Result};
use euclid::default::Point3D;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), str::parse::<i32>)(input)
}

fn parse_beacon(input: &str) -> IResult<&str, Point3D<i32>> {
    let (input, (x, _, y, _, z)) =
        tuple((parse_number, tag(","), parse_number, tag(","), parse_number))(input)?;

    Ok((input, Point3D::new(x, y, z)))
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point3D<i32>>,
}

fn parse_scanner(input: &str) -> IResult<&str, Scanner> {
    let (input, _) = tag("--- scanner ")(input)?;
    let (input, _id) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = tag(" ---")(input)?;
    let (input, _) = newline(input)?;

    let (input, beacons) = separated_list1(newline, parse_beacon)(input)?;

    Ok((input, Scanner { beacons }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Scanner>> {
    separated_list1(multispace1, parse_scanner)(input)
}

fn locate_scanner(
    scanner: &Scanner,
    located_scanners: &[Scanner],
) -> Option<(Point3D<i32>, Vec<Point3D<i32>>)> {
    let transforms = [
        |p: &Point3D<i32>| Point3D::new(p.x, p.y, p.z),
        |p: &Point3D<i32>| Point3D::new(p.x, p.z, -p.y),
        |p: &Point3D<i32>| Point3D::new(p.x, -p.y, -p.z),
        |p: &Point3D<i32>| Point3D::new(p.x, -p.z, p.y),
        |p: &Point3D<i32>| Point3D::new(p.y, p.x, -p.z),
        |p: &Point3D<i32>| Point3D::new(p.y, p.z, p.x),
        |p: &Point3D<i32>| Point3D::new(p.y, -p.x, p.z),
        |p: &Point3D<i32>| Point3D::new(p.y, -p.z, -p.x),
        |p: &Point3D<i32>| Point3D::new(p.z, p.x, p.y),
        |p: &Point3D<i32>| Point3D::new(p.z, p.y, -p.x),
        |p: &Point3D<i32>| Point3D::new(p.z, -p.x, -p.y),
        |p: &Point3D<i32>| Point3D::new(p.z, -p.y, p.x),
        |p: &Point3D<i32>| Point3D::new(-p.x, p.y, -p.z),
        |p: &Point3D<i32>| Point3D::new(-p.x, p.z, p.y),
        |p: &Point3D<i32>| Point3D::new(-p.x, -p.y, p.z),
        |p: &Point3D<i32>| Point3D::new(-p.x, -p.z, -p.y),
        |p: &Point3D<i32>| Point3D::new(-p.y, p.x, p.z),
        |p: &Point3D<i32>| Point3D::new(-p.y, p.z, -p.x),
        |p: &Point3D<i32>| Point3D::new(-p.y, -p.x, -p.z),
        |p: &Point3D<i32>| Point3D::new(-p.y, -p.z, p.x),
        |p: &Point3D<i32>| Point3D::new(-p.z, p.x, -p.y),
        |p: &Point3D<i32>| Point3D::new(-p.z, p.y, p.x),
        |p: &Point3D<i32>| Point3D::new(-p.z, -p.x, p.y),
        |p: &Point3D<i32>| Point3D::new(-p.z, -p.y, -p.x),
    ];

    for known_scanner in located_scanners {
        for transform in &transforms {
            let transformed_points = scanner.beacons.iter().map(transform).collect_vec();

            for offset in transformed_points
                .iter()
                .cartesian_product(&known_scanner.beacons)
                .map(|(a, b)| *a - *b)
            {
                let test_points = transformed_points.iter().map(|p| *p - offset).collect_vec();
                let matching = test_points
                    .iter()
                    .filter(|p| known_scanner.beacons.contains(p))
                    .collect_vec();

                if matching.len() >= 12 {
                    return Some((offset.to_point(), test_points));
                }
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/day19")?;
    let scanners = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let (first_scanner, scanners) = scanners.split_first().unwrap();

    // List of scanners whose locations we know. Beacon coordinates are absolute.
    let mut located_scanners = vec![first_scanner.clone()];

    // List of scanners we still need to locate.
    let mut unlocated_scanners = scanners.iter().cloned().collect::<VecDeque<Scanner>>();

    // Set of located beacons in absolute coordinates.
    let mut known_beacons: HashSet<Point3D<i32>> = HashSet::new();
    known_beacons.extend(first_scanner.beacons.iter());

    // Location of each scanner in absolute coordinates
    let mut scanner_offsets = vec![Point3D::new(0, 0, 0)];

    while let Some(mut scanner) = unlocated_scanners.pop_front() {
        match locate_scanner(&scanner, &located_scanners) {
            Some((scanner_offset, found_beacons)) => {
                known_beacons.extend(found_beacons.clone());
                scanner.beacons = found_beacons;
                located_scanners.push(scanner);
                scanner_offsets.push(scanner_offset);
            }
            None => {
                // Couldn't find match. Try again after the remaining scanners.
                unlocated_scanners.push_back(scanner);
            }
        }
    }

    let result_a = known_beacons.len();
    assert_eq!(result_a, 326);
    println!("Day 19, part A: {}", result_a);

    let result_b = scanner_offsets
        .iter()
        .combinations(2)
        .map(|scanners| {
            let offset = *scanners[1] - *scanners[0];
            offset.x.abs() + offset.y.abs() + offset.z.abs()
        })
        .sorted()
        .last()
        .unwrap();
    assert_eq!(result_b, 10630);
    println!("Day 19, part B: {}", result_b);

    Ok(())
}
