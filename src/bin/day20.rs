use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, newline},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

use anyhow::{anyhow, Result};
use bitvec::{field::BitField, prelude as bv};

#[derive(Debug, Clone)]
struct Image {
    width: usize,
    height: usize,
    bits: bv::BitVec<bv::Msb0>,
    extra_bits: bool,
}

impl Image {
    fn pad(&mut self) {
        let pad_top = self.bits[..self.width]
            .iter()
            .any(|b| *b != self.extra_bits);
        let pad_bottom = self.bits[((self.height - 1) * self.width)..]
            .iter()
            .any(|b| *b != self.extra_bits);
        let pad_left = self
            .bits
            .iter()
            .step_by(self.width)
            .any(|b| *b != self.extra_bits);
        let pad_right = self
            .bits
            .iter()
            .skip(self.width - 1)
            .step_by(self.width)
            .any(|b| *b != self.extra_bits);

        let old_width = self.width;
        self.width += pad_left as usize + pad_right as usize;
        self.height += pad_top as usize + pad_bottom as usize;

        if pad_top || pad_bottom || pad_left || pad_right {
            let mut bits = bv::BitVec::new();
            if pad_top {
                bits.extend(std::iter::once(self.extra_bits).cycle().take(self.width));
            }
            for row in self.bits.chunks(old_width) {
                if pad_left {
                    bits.push(self.extra_bits);
                }
                bits.extend(row);
                if pad_right {
                    bits.push(self.extra_bits);
                }
            }
            if pad_bottom {
                bits.extend(std::iter::once(self.extra_bits).cycle().take(self.width));
            }
            self.bits = bits;
        }
    }

    fn enhance(&mut self, algorithm: &bv::BitVec) {
        self.pad();

        let width = self.width;
        let height = self.height;

        let bits = (0..height)
            .flat_map(move |y| (0..width).map(move |x| (x, y)))
            .map(|(x, y)| {
                let lookup = ((y as isize - 1)..=(y as isize + 1))
                    .flat_map(move |y| ((x as isize - 1)..=(x as isize + 1)).map(move |x| (x, y)))
                    .map(|(x, y)| self.get_bit(x, y))
                    .collect::<bv::BitVec<bv::Msb0>>();

                let lookup = lookup.load_be::<usize>();
                algorithm[lookup]
            })
            .collect::<bv::BitVec<bv::Msb0>>();

        self.bits = bits;

        let extra_loookup = if self.extra_bits { (2 ^ 9) - 1 } else { 0 };
        self.extra_bits = algorithm[extra_loookup];
    }

    fn get_bit(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            self.extra_bits
        } else {
            self.bits[x as usize + y as usize * self.width]
        }
    }

    #[allow(dead_code)]
    fn draw(&self) {
        for i in 0..self.height {
            let string = self.bits[(i * self.width)..((i + 1) * self.width)]
                .iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>();
            println!("{}", &string);
        }
    }
}
fn parse_bit(input: &str) -> IResult<&str, bool> {
    alt((map(tag("."), |_| false), map(tag("#"), |_| true)))(input)
}

fn parse_algorithm(input: &str) -> IResult<&str, bv::BitVec> {
    let (input, bits) = many1(parse_bit)(input)?;

    Ok((input, bits.iter().collect()))
}

fn parse_image(input: &str) -> IResult<&str, Image> {
    let (input, rows) = separated_list1(newline, many1(parse_bit))(input)?;

    let width = rows[0].len();
    let height = rows.len();
    let bits = rows.iter().flatten().collect();
    Ok((
        input,
        Image {
            width,
            height,
            bits,
            extra_bits: false,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, (bv::BitVec, Image)> {
    let (input, algorithm) = parse_algorithm(input)?;
    let (input, _) = multispace1(input)?;
    let (input, image) = parse_image(input)?;

    Ok((input, (algorithm, image)))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/day20")?;
    let (algorithm, image) = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let mut image1 = image.clone();
    (0..2).for_each(|_| image1.enhance(&algorithm));
    let result_a = image1.bits.count_ones();
    assert_eq!(result_a, 5179);
    println!("Day 20, part A: {}", result_a);

    let mut image2 = image;
    (0..50).for_each(|_| image2.enhance(&algorithm));
    let result_b = image2.bits.count_ones();
    assert_eq!(result_b, 16112);
    println!("Day 20, part B: {}", result_b);

    Ok(())
}
