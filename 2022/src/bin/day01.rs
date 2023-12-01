#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

use std::fs::read_to_string;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Elf {
    cals: Vec<usize>,
}

impl Elf {
    fn from_input(input: &str) -> Result<Self> {
        let cals: std::result::Result<_, _> = input.split('\n').map(str::parse).collect();

        Ok(Self { cals: cals? })
    }

    fn total_cal(&self) -> usize {
        self.cals.iter().sum()
    }
}

fn parse_input(input: &str) -> Result<Vec<Elf>> {
    input.trim().split("\n\n").map(Elf::from_input).collect()
}

fn part1(elfs: &[Elf]) -> Option<usize> {
    elfs.iter().map(Elf::total_cal).max()
}

fn part2(elfs: &[Elf]) -> Option<usize> {
    if elfs.len() < 3 {
        return None;
    }

    let mut elf_cals: Vec<_> = elfs.iter().map(Elf::total_cal).collect();
    elf_cals.sort_unstable();

    Some(elf_cals[elf_cals.len() - 3..].iter().sum())
}

fn main() -> Result<()> {
    let input = read_to_string("input/day01.txt")?;
    let elfs = parse_input(&input)?;

    let part1 = part1(&elfs).ok_or_else(|| anyhow!("no elfs!"))?;
    let part2 = part2(&elfs).ok_or_else(|| anyhow!("not enough elfs!"))?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

        let elfs = parse_input(input).unwrap();

        assert_eq!(
            elfs[0],
            Elf {
                cals: vec![1000, 2000, 3000]
            }
        );
        assert_eq!(elfs[4], Elf { cals: vec![10000] });

        assert_eq!(part1(&elfs), Some(24_000));
        assert_eq!(part2(&elfs), Some(45_000));
    }
}
