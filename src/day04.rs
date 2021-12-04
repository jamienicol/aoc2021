use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BingoCell {
    number: usize,
    x: usize,
    y: usize,
    checked: bool,
}

#[derive(Debug)]
struct BingoBoard {
    width: usize,
    height: usize,
    cells: Vec<BingoCell>,
    rows_checked: Vec<usize>,
    columns_checked: Vec<usize>,
    complete: bool,
}

impl BingoBoard {
    fn new(width: usize, height: usize, cells: Vec<BingoCell>) -> Self {
        Self {
            width,
            height,
            cells,
            rows_checked: vec![0; height],
            columns_checked: vec![0; width],
            complete: false,
        }
    }

    fn score(&self) -> usize {
        self.cells
            .iter()
            .filter_map(|cell| (!cell.checked).then(|| cell.number))
            .sum()
    }
}

#[derive(Debug)]
struct BingoGame {
    draw: Vec<usize>,
    boards: Vec<BingoBoard>,
}

fn parse_draw(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), map_res(digit1, |s: &str| s.parse::<usize>()))(input)
}

fn parse_board(input: &str) -> IResult<&str, BingoBoard> {
    let (input, grid) = separated_list1(
        newline,
        separated_list1(
            space1,
            preceded(space0, map_res(digit1, |s: &str| s.parse::<usize>())),
        ),
    )(input)?;

    let height = grid.len();
    let width = grid[0].len();

    let mut cells = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            cells.push(BingoCell {
                number: *val,
                x,
                y,
                checked: false,
            });
        }
    }
    cells.sort_unstable();

    Ok((input, BingoBoard::new(width, height, cells)))
}

fn parse_game(input: &str) -> IResult<&str, BingoGame> {
    let (input, draw) = parse_draw(input)?;

    let (input, _space) = multispace1(input)?;

    let (input, boards) = separated_list1(multispace1, parse_board)(input)?;

    Ok((input, BingoGame { draw, boards }))
}

pub fn part_a() -> Result<usize> {
    let input = std::fs::read_to_string("res/day04")?;

    let (_, mut game) = parse_game(&input).map_err(|e| anyhow!("{:?}", e))?;

    for number in game.draw {
        for board in game.boards.iter_mut() {
            if let Some(cell) = board.cells.iter_mut().find(|cell| cell.number == number) {
                board.rows_checked[cell.y] += 1;
                board.columns_checked[cell.x] += 1;
                cell.checked = true;

                if board.rows_checked[cell.y] == board.width
                    || board.columns_checked[cell.x] == board.height
                {
                    return Ok(board.score() * number);
                }
            }
        }
    }

    Err(anyhow!("Could not find winning board"))
}

pub fn part_b() -> Result<usize> {
    let input = std::fs::read_to_string("res/day04")?;

    let (_, mut game) = parse_game(&input).map_err(|e| anyhow!("{:?}", e))?;

    let mut last_winner = None;
    for number in game.draw {
        for board in game.boards.iter_mut() {
            if !board.complete {
                if let Some(cell) = board.cells.iter_mut().find(|cell| cell.number == number) {
                    board.rows_checked[cell.y] += 1;
                    board.columns_checked[cell.x] += 1;
                    cell.checked = true;

                    if board.rows_checked[cell.y] == board.width
                        || board.columns_checked[cell.x] == board.height
                    {
                        board.complete = true;
                        last_winner = Some(board.score() * number);
                    }
                }
            }
        }
    }

    last_winner.ok_or_else(|| anyhow!("Could not find winning board"))
}
