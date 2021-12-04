use anyhow::Result;

mod day01;
mod day02;
mod day03;

fn main() -> Result<()> {
    let result_1a = day01::part_a()?;
    println!("Day 1, part A: {}", result_1a);

    let result_1b = day01::part_b()?;
    println!("Day 1, part B: {}", result_1b);

    let result_2a = day02::part_a()?;
    println!("Day 2, part A: {}", result_2a);

    let result_2b = day02::part_b()?;
    println!("Day 2, part B: {}", result_2b);

    let result_3a = day03::part_a()?;
    println!("Day 3, part A: {}", result_3a);

    let result_3b = day03::part_b()?;
    println!("Day 3, part B: {}", result_3b);

    Ok(())
}
