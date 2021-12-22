use std::{cell::RefCell, fmt, ops::Add, rc::Rc};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(PartialEq)]
enum Number {
    Digit(u32),
    Pair(Rc<RefCell<Number>>, Rc<RefCell<Number>>),
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut res = Self::Pair(Rc::new(RefCell::new(self)), Rc::new(RefCell::new(other)));
        res.reduce();
        res
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Digit(digit) => write!(f, "{}", digit),
            Number::Pair(left, right) => write!(
                f,
                "[{:?},{:?}]",
                left.as_ref().borrow(),
                right.as_ref().borrow()
            ),
        }
    }
}

impl Clone for Number {
    fn clone(&self) -> Self {
        match self {
            Number::Digit(d) => Number::Digit(*d),
            Number::Pair(a, b) => Number::Pair(
                Rc::new(RefCell::new(a.borrow().clone())),
                Rc::new(RefCell::new(b.borrow().clone())),
            ),
        }
    }
}

impl Number {
    fn as_digit(&self) -> u32 {
        match self {
            Number::Digit(d) => *d,
            Number::Pair(_, _) => panic!("Not a digit"),
        }
    }

    fn as_pair(&self) -> (&Rc<RefCell<Number>>, &Rc<RefCell<Number>>) {
        match self {
            Number::Digit(_) => panic!("Not a pair"),
            Number::Pair(left, right) => (left, right),
        }
    }

    fn try_explode(&mut self) -> bool {
        let mut prev_digit = None;
        let mut next_digit = None;
        let mut to_explode = None;

        let mut to_search: Vec<(usize, Rc<RefCell<Number>>)> = match self {
            Number::Digit(_) => vec![],
            Number::Pair(a, b) => vec![(1, Rc::clone(b)), (1, Rc::clone(a))],
        };

        while let Some((depth, node)) = to_search.pop() {
            let node_borrow = node.borrow();
            match &*node_borrow {
                Number::Digit(_) => {
                    prev_digit = Some(Rc::clone(&node));
                }
                Number::Pair(a, b) => {
                    let a_borrow = a.borrow();
                    let b_borrow = b.borrow();
                    match (&*a_borrow, &*b_borrow) {
                        (Number::Digit(_), Number::Digit(_)) if depth >= 4 => {
                            next_digit = loop {
                                if let Some((_, node)) = to_search.pop() {
                                    let node_borrow = node.borrow();
                                    match &*node_borrow {
                                        Number::Digit(_) => {
                                            break Some(Rc::clone(&node));
                                        }
                                        Number::Pair(a, b) => {
                                            to_search.push((depth + 1, Rc::clone(b)));
                                            to_search.push((depth + 1, Rc::clone(a)));
                                        }
                                    }
                                } else {
                                    break None;
                                }
                            };

                            to_explode = Some(Rc::clone(&node));
                            break;
                        }
                        _ => {
                            to_search.push((depth + 1, Rc::clone(b)));
                            to_search.push((depth + 1, Rc::clone(a)));
                        }
                    }
                }
            }
        }

        if let Some(node) = to_explode {
            let mut node_borrow = node.borrow_mut();

            if let Some(prev_digit) = prev_digit {
                let mut prev_borrow = prev_digit.borrow_mut();
                *prev_borrow = Number::Digit(
                    prev_borrow.as_digit() + node_borrow.as_pair().0.borrow().as_digit(),
                );
            }
            if let Some(next_digit) = next_digit {
                let mut next_borrow = next_digit.borrow_mut();
                *next_borrow = Number::Digit(
                    next_borrow.as_digit() + node_borrow.as_pair().1.borrow().as_digit(),
                );
            }

            *node_borrow = Number::Digit(0);

            true
        } else {
            false
        }
    }

    fn try_split(&mut self) -> bool {
        let mut to_search: Vec<Rc<RefCell<Number>>> = match self {
            Number::Digit(_) => vec![],
            Number::Pair(a, b) => vec![Rc::clone(b), Rc::clone(a)],
        };

        while let Some(node) = to_search.pop() {
            let mut node_borrow = node.borrow_mut();
            match &*node_borrow {
                Number::Digit(d) => {
                    if *d >= 10 {
                        let val = node_borrow.as_digit();
                        let left = Rc::new(RefCell::new(Number::Digit(val / 2)));
                        let right = Rc::new(RefCell::new(Number::Digit((val + 1) / 2)));
                        *node_borrow = Number::Pair(left, right);
                        return true;
                    }
                }
                Number::Pair(a, b) => {
                    to_search.push(Rc::clone(b));
                    to_search.push(Rc::clone(a));
                }
            }
        }

        false
    }

    fn reduce(&mut self) {
        while self.try_explode() || self.try_split() {}
    }

    fn magnitude(&self) -> u32 {
        match self {
            Number::Digit(d) => *d,
            Number::Pair(left, right) => {
                left.borrow().magnitude() * 3 + right.borrow().magnitude() * 2
            }
        }
    }
}

fn parse_digit(input: &str) -> IResult<&str, Number> {
    let (input, digit) = map_res(digit1, str::parse::<u32>)(input)?;

    Ok((input, Number::Digit(digit)))
}

fn parse_pair(input: &str) -> IResult<&str, Number> {
    let (input, pair) = terminated(
        preceded(
            tag("["),
            separated_pair(parse_number, tag(","), parse_number),
        ),
        tag("]"),
    )(input)?;

    Ok((
        input,
        Number::Pair(Rc::new(RefCell::new(pair.0)), Rc::new(RefCell::new(pair.1))),
    ))
}

fn parse_number(input: &str) -> IResult<&str, Number> {
    alt((parse_digit, parse_pair))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Number>> {
    separated_list1(newline, parse_number)(input)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/day18")?;
    let numbers = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = numbers[1..]
        .iter()
        .fold(numbers[0].clone(), |acc, num| acc + num.clone())
        .magnitude();
    assert_eq!(result_a, 3051);
    println!("Day 18, part A: {}", result_a);

    let result_b = numbers
        .into_iter()
        .permutations(2)
        .map(|p| (p[0].clone() + p[1].clone()).magnitude())
        .max()
        .unwrap();
    assert_eq!(result_b, 4812);
    println!("Day 18, part B: {}", result_b);

    Ok(())
}
