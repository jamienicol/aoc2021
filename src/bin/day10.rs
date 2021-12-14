use core::panic;

use anyhow::Result;

fn matching_opener(closer: char) -> char {
    match closer {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Unexpected closer {:?}", closer),
    }
}

fn matching_closer(opener: char) -> char {
    match opener {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unexpected opener {:?}", opener),
    }
}

fn invalid_score(closer: char) -> u32 {
    match closer {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unexpected closer {:?}", closer),
    }
}

fn incomplete_score(closer: char) -> u64 {
    match closer {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Unexpected closer {:?}", closer),
    }
}

fn part_a(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut open_chunks = Vec::new();
            let mut score = 0;
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => open_chunks.push(c),
                    ')' | ']' | '}' | '>' => {
                        let opener = open_chunks.pop();
                        if opener != Some(matching_opener(c)) {
                            score = invalid_score(c);
                            break;
                        }
                    }
                    _ => panic!("Unexpected char {:?}", c),
                }
            }
            score
        })
        .sum()
}

fn part_b(input: &str) -> u64 {
    let incomplete = input.lines().filter_map(|line| {
        let mut open_chunks = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => open_chunks.push(c),
                ')' | ']' | '}' | '>' => {
                    let opener = open_chunks.pop();
                    if opener != Some(matching_opener(c)) {
                        return None;
                    }
                }
                _ => panic!("Unexpected char {:?}", c),
            }
        }
        Some(
            open_chunks
                .into_iter()
                .rev()
                .map(matching_closer)
                .collect::<Vec<_>>(),
        )
    });

    let mut scores = incomplete
        .map(|line| line.iter().fold(0, |acc, c| acc * 5 + incomplete_score(*c)))
        .collect::<Vec<_>>();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() -> Result<()> {
    let input = &std::fs::read_to_string("res/day10")?;

    let result_a = part_a(input);
    assert_eq!(result_a, 392043);
    println!("Day 10, part A: {}", result_a);

    let result_b = part_b(input);
    assert_eq!(result_b, 1605968119);
    println!("Day 10, part B: {}", result_b);

    Ok(())
}
