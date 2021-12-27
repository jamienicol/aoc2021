use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res, opt, recognize, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone)]
enum Variable {
    X,
    Y,
    Z,
    W,
}

impl Index<&Variable> for [i64; 4] {
    type Output = i64;

    fn index(&self, index: &Variable) -> &Self::Output {
        match index {
            Variable::W => &self[0],
            Variable::X => &self[1],
            Variable::Y => &self[2],
            Variable::Z => &self[3],
        }
    }
}

impl IndexMut<&Variable> for [i64; 4] {
    fn index_mut(&mut self, index: &Variable) -> &mut Self::Output {
        match index {
            Variable::W => &mut self[0],
            Variable::X => &mut self[1],
            Variable::Y => &mut self[2],
            Variable::Z => &mut self[3],
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Variable(Variable),
    Literal(i64),
}

#[derive(Debug, Clone)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Operand),
    Mul(Variable, Operand),
    Div(Variable, Operand),
    Mod(Variable, Operand),
    Eql(Variable, Operand),
}

fn parse_var(input: &str) -> IResult<&str, Variable> {
    alt((
        value(Variable::W, tag("w")),
        value(Variable::X, tag("x")),
        value(Variable::Y, tag("y")),
        value(Variable::Z, tag("z")),
    ))(input)
}

fn parse_num(input: &str) -> IResult<&str, i64> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), str::parse::<i64>)(input)
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(parse_var, Operand::Variable),
        map(parse_num, Operand::Literal),
    ))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(preceded(tag("inp "), parse_var), |var| {
            Instruction::Inp(var)
        }),
        map(
            preceded(
                tag("add "),
                separated_pair(parse_var, tag(" "), parse_operand),
            ),
            |(var, op)| Instruction::Add(var, op),
        ),
        map(
            preceded(
                tag("mul "),
                separated_pair(parse_var, tag(" "), parse_operand),
            ),
            |(var, op)| Instruction::Mul(var, op),
        ),
        map(
            preceded(
                tag("div "),
                separated_pair(parse_var, tag(" "), parse_operand),
            ),
            |(var, op)| Instruction::Div(var, op),
        ),
        map(
            preceded(
                tag("mod "),
                separated_pair(parse_var, tag(" "), parse_operand),
            ),
            |(var, op)| Instruction::Mod(var, op),
        ),
        map(
            preceded(
                tag("eql "),
                separated_pair(parse_var, tag(" "), parse_operand),
            ),
            |(var, op)| Instruction::Eql(var, op),
        ),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, parse_instruction)(input)
}

#[derive(Debug)]
struct Alu {
    program: Vec<Instruction>,
    input: Vec<u64>,
    variables: [i64; 4],
}

impl Alu {
    fn new(program: &[Instruction], input: &[u64], z: i64) -> Self {
        let mut variables = [0; 4];
        variables[&Variable::Z] = z;

        Alu {
            program: program.to_vec(),
            input: input.iter().rev().cloned().collect::<Vec<_>>(),
            variables,
        }
    }

    /// Executes the list of instructions, returning the value of the Z
    /// variables once complete.
    fn run(&mut self) -> i64 {
        for instr in &self.program {
            match instr {
                Instruction::Inp(var) => {
                    let val = self.input.pop().unwrap();
                    self.variables[var] = val as i64;
                }
                Instruction::Add(lhs, rhs) => {
                    let rhs = match rhs {
                        Operand::Literal(l) => *l,
                        Operand::Variable(v) => self.variables[v],
                    };
                    self.variables[lhs] += rhs;
                }
                Instruction::Mul(lhs, rhs) => {
                    let rhs = match rhs {
                        Operand::Literal(l) => *l,
                        Operand::Variable(v) => self.variables[v],
                    };
                    self.variables[lhs] *= rhs;
                }
                Instruction::Div(lhs, rhs) => {
                    let rhs = match rhs {
                        Operand::Literal(l) => *l,
                        Operand::Variable(v) => self.variables[v],
                    };
                    self.variables[lhs] /= rhs;
                }
                Instruction::Mod(lhs, rhs) => {
                    let rhs = match rhs {
                        Operand::Literal(l) => *l,
                        Operand::Variable(v) => self.variables[v],
                    };
                    self.variables[lhs] %= rhs;
                }
                Instruction::Eql(lhs, rhs) => {
                    let rhs = match rhs {
                        Operand::Literal(l) => *l,
                        Operand::Variable(v) => self.variables[v],
                    };
                    self.variables[lhs] = (self.variables[lhs] == rhs) as i64;
                }
            }
        }

        self.variables[&Variable::Z]
    }
}
fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/day24")?;
    let program = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    // Looking at the input, we can see that the program is split in to 18 chunks.
    // Each one reads a single input in to register W, and clears X and Y before use.
    // Therefore Z is the only register whose value affects the result of the chunk.
    // To find the valid model numbers, we therefore iterate through each program chunk,
    // and will execute it for each possible combination of input and previous Z value.
    // At each stage we save the minimum and maximum inut values which have resulted in
    // each Z value.
    // Once we have processed all 14 chunks, we look up the input values which resulted
    // in a Z value of 0, and that gives us our min and max model numbers.
    let mut prev_zs = HashMap::new();
    prev_zs.insert(0, (0, 0));
    for program_chunk in program.chunks(18) {
        let mut new_zs = HashMap::new();

        for (prev_z, new_input) in prev_zs.keys().cartesian_product(1..=9) {
            let mut alu = Alu::new(program_chunk, &[new_input], *prev_z);
            let new_z = alu.run();

            let (mut min_input, mut max_input) = prev_zs.get(prev_z).cloned().unwrap_or_default();
            min_input = min_input * 10 + new_input;
            max_input = max_input * 10 + new_input;
            new_zs
                .entry(new_z)
                .and_modify(|(existing_min, existing_max)| {
                    if min_input < *existing_min {
                        *existing_min = min_input;
                    }
                    if max_input > *existing_max {
                        *existing_max = max_input;
                    }
                })
                .or_insert((min_input, max_input));
        }
        prev_zs = new_zs;
    }

    let (min, max) = prev_zs
        .get(&0)
        .ok_or_else(|| anyhow!("Failed to find valid model number"))?;

    let result_a = *max;
    assert_eq!(result_a, 12996997829399);
    println!("Day 24, part A: {}", result_a);

    let result_b = *min;
    assert_eq!(result_b, 11841231117189);
    println!("Day 24, part B: {}", result_b);

    Ok(())
}
