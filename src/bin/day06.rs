use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};

const NEW_FISH_DAYS_UNTIL_SPAWN: usize = 8;
const OLD_FISH_DAYS_UNTIL_SPAWN: usize = 6;

type School = [usize; NEW_FISH_DAYS_UNTIL_SPAWN + 1];

// Parse the input, each value representing the number of days until it spawns
fn parse_fish(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), map_res(digit1, str::parse::<usize>))(input)
}

fn simulate(fish: &mut School, n: usize) {
    for _ in 0..n {
        // Each fish becomes one day closer to spawning so rotate left.
        // This will add the *new fish* spawned by each fish that was 0 days to the end.
        fish.rotate_left(1);
        // But we must manually add the fish themselves that were on 0 days to the right bucket.
        fish[OLD_FISH_DAYS_UNTIL_SPAWN] += fish[NEW_FISH_DAYS_UNTIL_SPAWN]
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/day06")?;

    let mut school = parse_fish(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1
        .iter()
        .try_fold(School::default(), |mut school, age| match *age {
            age if age < school.len() => {
                school[age] += 1;
                Ok(school)
            }
            _ => Err(anyhow!("Unexpected input {}", age)),
        })?;

    simulate(&mut school, 80);
    let result_a: usize = school.iter().sum();
    assert_eq!(result_a, 362740);
    println!("Day 6, part A: {}", result_a);

    simulate(&mut school, 256 - 80);
    let result_b: usize = school.iter().sum();
    assert_eq!(result_b, 1644874076764);
    println!("Day 6, part B: {}", result_b);

    Ok(())
}
