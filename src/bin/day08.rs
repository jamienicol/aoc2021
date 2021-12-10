use std::convert::TryInto;

use anyhow::{anyhow, Error, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn index_from_char(c: char) -> Result<u8> {
    match c {
        c if ('a'..='g').contains(&c) => Ok(c as u8 - b'a'),
        _ => Err(anyhow!("{} is not a valid segment", c)),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Digit {
    segments: u8,
}

impl Digit {
    fn from_str(input: &str) -> Result<Self> {
        let segments = input
            .chars()
            .try_fold(0, |acc, c| {
                let index = index_from_char(c)?;

                Ok(acc | (1 << index))
            })
            .map_err(|e: Error| anyhow! {"Error parsing digit: {:?}", e})?;

        Ok(Self { segments })
    }

    fn count_segments(&self) -> usize {
        (0..7)
            .map(|n| (self.segments as usize & (1 << n)) >> n)
            .sum()
    }
}

#[derive(Debug)]
struct Display {
    patterns: [Digit; 10],
    output: [Digit; 4],
}

fn parse_display(input: &str) -> IResult<&str, Display> {
    let (input, (patterns, output)) = separated_pair(
        separated_list1(space1, map_res(alpha1, Digit::from_str)),
        tag(" | "),
        separated_list1(space1, map_res(alpha1, Digit::from_str)),
    )(input)?;

    let patterns: [Digit; 10] = patterns.try_into().unwrap();
    let output: [Digit; 4] = output.try_into().unwrap();

    Ok((input, Display { patterns, output }))
}
fn parse_input(input: &str) -> IResult<&str, Vec<Display>> {
    separated_list1(newline, parse_display)(input)
}

fn part_a(displays: &[Display]) -> usize {
    displays
        .iter()
        .flat_map(|display| &display.output)
        .map(|digit| match digit.count_segments() {
            2 | 3 | 4 | 7 => 1,
            _ => 0,
        })
        .sum::<usize>()
}

fn part_b(displays: &[Display]) -> usize {
    displays
        .iter()
        .map(|display| {
            let mut digits = [None; 10];

            let mut rev_digits = [None; 128];

            for digit in display.patterns.iter().chain(&display.output).cycle() {
                let num_segments = digit.count_segments();
                if digits[0].is_none() && num_segments == 6 {
                    if let (Some(six), Some(nine)) = (digits[6], digits[9]) {
                        if *digit != six && *digit != nine {
                            digits[0] = Some(*digit);
                        }
                    }
                }
                if digits[1].is_none() && num_segments == 2 {
                    digits[1] = Some(*digit);
                }

                if digits[2].is_none() && num_segments == 5 {
                    if let (Some(three), Some(five)) = (digits[3], digits[5]) {
                        if *digit != three && *digit != five {
                            digits[2] = Some(*digit);
                        }
                    }
                }

                if digits[3].is_none() && num_segments == 5 {
                    if let Some(seven) = digits[7] {
                        if (digit.segments & seven.segments) == seven.segments {
                            digits[3] = Some(*digit);
                        }
                    }
                }

                if digits[4].is_none() && num_segments == 4 {
                    digits[4] = Some(*digit);
                }

                if digits[5].is_none() && num_segments == 5 {
                    if let (Some(three), Some(nine)) = (digits[3], digits[9]) {
                        if digit.segments | nine.segments == nine.segments && *digit != three {
                            digits[5] = Some(*digit);
                        }
                    }
                }

                if digits[6].is_none() && num_segments == 6 {
                    if let (Some(five), Some(nine)) = (digits[5], digits[9]) {
                        if digit.segments & five.segments == five.segments && *digit != nine {
                            digits[6] = Some(*digit);
                        }
                    }
                }

                if digits[7].is_none() && num_segments == 3 {
                    digits[7] = Some(*digit);
                }

                if digits[8].is_none() && num_segments == 7 {
                    digits[8] = Some(*digit);
                }

                if digits[9].is_none() && num_segments == 6 {
                    if let Some(three) = digits[3] {
                        if digit.segments & three.segments == three.segments {
                            digits[9] = Some(*digit);
                        }
                    }
                }

                if digits.iter().all(|d| d.is_some()) {
                    break;
                }
            }

            for (i, digit) in digits.iter().enumerate() {
                rev_digits[digit.unwrap().segments as usize] = Some(i);
            }

            let output = display
                .output
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, d)| {
                    acc + (rev_digits[d.segments as usize].unwrap() * 10usize.pow(i as u32))
                });

            output
        })
        .sum()
}

fn main() -> Result<()> {
    let input = &std::fs::read_to_string("res/day08")?;

    let displays = parse_input(input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = part_a(&displays);
    assert_eq!(result_a, 440);
    println!("Day 8, part A: {}", result_a);

    let result_b = part_b(&displays);
    println!("Day 8, part B: {}", result_b);
    assert_eq!(result_b, 1046281);

    Ok(())
}
