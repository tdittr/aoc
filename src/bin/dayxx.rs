use anyhow::Result;
use std::fmt::Display;
use std::fs::read_to_string;

type Input = usize;

fn parse_input(_input: &str) -> Result<Input> {
    todo!()
}

fn part1(_g: &Input) -> Result<impl Display> {
    Ok("todo")
}

fn part2(_g: &Input) -> Result<impl Display> {
    Ok("todo")
}

fn main() -> Result<()> {
    let input = read_to_string("input/day05.txt").unwrap();
    let input = parse_input(&input)?;

    let part1 = part1(&input)?;
    println!("Part 1: {part1}");

    let part2 = part2(&input)?;
    println!("Part 2: {part2}");

    Ok(())
}
