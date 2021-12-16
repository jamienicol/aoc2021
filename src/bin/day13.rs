use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline, one_of},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
struct Fold {
    orientation: Orientation,
    position: usize,
}

fn parse_dots(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(
        newline,
        separated_pair(
            map_res(digit1, str::parse::<usize>),
            tag(","),
            map_res(digit1, str::parse::<usize>),
        ),
    )(input)
}

fn parse_folds(input: &str) -> IResult<&str, Vec<Fold>> {
    separated_list1(
        newline,
        preceded(
            tag("fold along "),
            map(
                separated_pair(
                    map_res(one_of("xy"), |c| match c {
                        'x' => Ok(Orientation::Horizontal),
                        'y' => Ok(Orientation::Vertical),
                        _ => Err(anyhow!("Unexpected axis {}", c)),
                    }),
                    tag("="),
                    map_res(digit1, str::parse::<usize>),
                ),
                |(orientation, position)| Fold {
                    orientation,
                    position,
                },
            ),
        ),
    )(input)
}

fn fold_dots(mut dots: Vec<(usize, usize)>, fold: Fold) -> Vec<(usize, usize)> {
    for dot in dots.iter_mut() {
        match fold.orientation {
            Orientation::Horizontal => {
                if dot.0 > fold.position {
                    dot.0 = fold.position - (dot.0 - fold.position)
                }
            }
            Orientation::Vertical => {
                if dot.1 > fold.position {
                    dot.1 = fold.position - (dot.1 - fold.position)
                }
            }
        }
    }

    dots.sort_unstable();
    dots.dedup();
    dots
}

fn to_ascii(dots: &[(usize, usize)]) -> String {
    let size = dots.iter().fold((0, 0), |size, dot| {
        (size.0.max(dot.0 + 1), size.1.max(dot.1 + 1))
    });
    println!("size: {:?}", size);
    (0..size.1)
        .map(|y| {
            let mut chars = vec!["."; size.0];
            for dot in dots {
                if dot.1 == y {
                    chars[dot.0] = "#";
                }
            }
            chars.into_iter().collect::<String>()
        })
        .collect_vec()
        .join("\n")
}

fn main() -> Result<()> {
    let input = &std::fs::read_to_string("res/day13")?;
    let (dots, folds) = separated_pair(parse_dots, multispace1, parse_folds)(input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    println!("dots: {:?}", dots);
    println!("folds: {:?}", folds);

    let result_a = fold_dots(dots.clone(), *folds.first().unwrap()).len();
    assert_eq!(result_a, 706);
    println!("Day 13, part A: {}", result_a);

    let final_dots = folds.iter().fold(dots, |dots, fold| fold_dots(dots, *fold));
    let result_b = to_ascii(&final_dots);
    assert_eq!(
        result_b,
        "\
#....###..####...##.###....##.####.#..#
#....#..#.#.......#.#..#....#.#....#..#
#....#..#.###.....#.###.....#.###..####
#....###..#.......#.#..#....#.#....#..#
#....#.#..#....#..#.#..#.#..#.#....#..#
####.#..#.#.....##..###...##..####.#..#"
    );
    println!("Day 13, part B:\n{}", result_b);

    Ok(())
}
