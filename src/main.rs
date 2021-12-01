use anyhow::Result;

mod day01;

fn main() -> Result<()> {
    let result_1a = day01::part_a()?;
    println!("Day 1, part A: {}", result_1a);

    let result_1b = day01::part_b()?;
    println!("Day 1, part B: {}", result_1b);

    Ok(())
}
