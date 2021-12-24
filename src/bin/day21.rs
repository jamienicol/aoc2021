use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map_res,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(
        preceded(
            tag("Player 1 starting position: "),
            map_res(digit1, str::parse::<usize>),
        ),
        newline,
        preceded(
            tag("Player 2 starting position: "),
            map_res(digit1, str::parse::<usize>),
        ),
    )(input)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Game {
    positions: [usize; 2],
    scores: [usize; 2],
    current_player: usize,
    target_score: usize,
}

impl Game {
    fn new(p1_start_pos: usize, p2_start_pos: usize, target_score: usize) -> Self {
        Game {
            positions: [p1_start_pos, p2_start_pos],
            scores: [0, 0],
            current_player: 0,
            target_score,
        }
    }

    fn winning_player(&self) -> Option<usize> {
        self.scores.iter().position(|s| *s >= self.target_score)
    }

    fn take_turn(&mut self, roll: usize) {
        self.positions[self.current_player] =
            ((self.positions[self.current_player] - 1 + roll) % 10) + 1;
        self.scores[self.current_player] += self.positions[self.current_player];
        self.current_player = (self.current_player + 1) % self.positions.len()
    }
}

fn part_a(p1_start_pos: usize, p2_start_pos: usize) -> usize {
    let mut game = Game::new(p1_start_pos, p2_start_pos, 1000);

    let dice = (1..=100).cycle().chunks(3);
    let mut dice_iter = dice.into_iter();
    let mut num_rolls = 0;

    while game.winning_player().is_none() {
        game.take_turn(dice_iter.next().unwrap().sum::<usize>());
        num_rolls += 3;
    }

    game.scores.iter().min().unwrap() * num_rolls
}

fn part_b(p1_start_pos: usize, p2_start_pos: usize) -> u64 {
    let mut games = HashMap::new();
    games.insert(Game::new(p1_start_pos, p2_start_pos, 21), 1);

    let mut wins = [0u64, 0];

    while !games.is_empty() {
        let old_games = games.drain().collect_vec();
        for (game, game_count) in old_games {
            let rolls = (1..=3)
                .cartesian_product(1..=3)
                .cartesian_product(1..=3)
                .map(|dice| dice.0 .0 + dice.0 .1 + dice.1)
                .counts()
                .into_iter()
                .sorted();
            for (roll, roll_count) in rolls {
                let mut split_game = game.clone();
                split_game.take_turn(roll);
                if let Some(winning_player) = split_game.winning_player() {
                    wins[winning_player] += game_count * roll_count as u64;
                } else {
                    *games.entry(split_game).or_default() += game_count * roll_count as u64;
                }
            }
        }
    }

    *wins.iter().max().unwrap()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/day21")?;
    let (p1_start_pos, p2_start_pos) = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = part_a(p1_start_pos, p2_start_pos);
    assert_eq!(result_a, 1006866);
    println!("Day 21, part A: {}", result_a);

    let result_b = part_b(p1_start_pos, p2_start_pos);
    assert_eq!(result_b, 273042027784929);
    println!("Day 21, part B: {}", result_b);

    Ok(())
}
