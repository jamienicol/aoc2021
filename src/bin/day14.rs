use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn parse_template(input: &str) -> IResult<&str, String> {
    map(alpha1, str::to_string)(input)
}

fn parse_rules(input: &str) -> IResult<&str, HashMap<(char, char), char>> {
    let (input, rules) =
        separated_list1(newline, separated_pair(alpha1, tag(" -> "), alpha1))(input)?;

    let mut map: HashMap<(char, char), char> = HashMap::default();
    for rule in rules {
        map.insert(
            rule.0
                .chars()
                .collect_tuple()
                .expect("Invalid length pair insertion rule source"),
            rule.1
                .chars()
                .next()
                .expect("Missing pair insertion rule dest"),
        );
    }

    Ok((input, map))
}

fn do_calculaton(template: &str, rules: &HashMap<(char, char), char>, num_steps: usize) -> usize {
    let polymer = template.chars().tuple_windows::<(char, char)>().counts();

    let counts = template.chars().counts();

    let (_polymer, counts) = (0..num_steps).fold((polymer, counts), |(polymer, mut counts), _| {
        let mut new_polymer = HashMap::new();
        for ((a, b), count) in polymer.iter() {
            if let Some(c) = rules.get(&(*a, *b)) {
                *new_polymer.entry((*a, *c)).or_insert(0) += count;
                *new_polymer.entry((*c, *b)).or_insert(0) += count;

                *counts.entry(*c).or_insert(0) += count;
            }
        }
        (new_polymer, counts)
    });

    let counts = counts.values().cloned().sorted().collect_vec();
    *counts.last().unwrap() - *counts.first().unwrap()
}

fn main() -> Result<()> {
    let input = &std::fs::read_to_string("res/day14")?;
    let (template, rules) = separated_pair(parse_template, multispace1, parse_rules)(input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = do_calculaton(&template, &rules, 10);
    assert_eq!(result_a, 2915);
    println!("Day 14, part A: {}", result_a);

    let result_b = do_calculaton(&template, &rules, 40);
    assert_eq!(result_b, 3353146900153);
    println!("Day 14, part B: {}", result_b);

    Ok(())
}
