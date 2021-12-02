use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    distance: usize,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("forward"), |_| Direction::Forward),
        map(tag("down"), |_| Direction::Down),
        map(tag("up"), |_| Direction::Up),
    ))(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, (direction, distance)) = separated_pair(
        parse_direction,
        space1,
        map_res(digit1, |s: &str| s.parse::<usize>()),
    )(input)?;

    Ok((
        input,
        Command {
            direction,
            distance,
        },
    ))
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(newline, parse_command)(input)
}

pub fn part_a() -> Result<usize> {
    let input = std::fs::read_to_string("res/day02")?;

    let (_, commands) = parse_commands(&input).map_err(|e| anyhow!("{:?}", e))?;

    let (x, y) = commands
        .iter()
        .fold((0, 0), |(x, y), command| match command.direction {
            Direction::Forward => (x + command.distance, y),
            Direction::Down => (x, y + command.distance),
            Direction::Up => (x, y - command.distance),
        });

    Ok(x * y)
}

pub fn part_b() -> Result<usize> {
    let input = std::fs::read_to_string("res/day02")?;

    let (_, commands) = parse_commands(&input).map_err(|e| anyhow!("{:?}", e))?;

    let ((x, y), _aim) =
        commands.iter().fold(((0, 0), 0), |((x, y), aim), command| {
            match command.direction {
                Direction::Forward => ((x + command.distance, y + aim * command.distance), aim),
                Direction::Down => ((x, y), aim + command.distance),
                Direction::Up => ((x, y), aim - command.distance),
            }
        });

    Ok(x * y)
}
