use anyhow::{anyhow, Result};
use std::fs::read_to_string;
use std::str::FromStr;

fn priority(item: u8) -> Option<u8> {
    match item {
        c @ b'a'..=b'z' => Some(c - b'a' + 1),
        c @ b'A'..=b'Z' => Some(c - b'A' + 27),
        _ => None,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Compartment(u64);

impl Compartment {
    fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    fn single_item(self) -> Result<u32> {
        let ones = self.0.count_ones();
        if ones != 1 {
            Err(anyhow!(
                "Not exactly one but {ones} items in compartment: {}",
                self.0
            ))
        } else {
            Ok(self.0.trailing_zeros())
        }
    }
}

impl FromStr for Compartment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = 0;
        for prio in s.bytes().map(priority) {
            mask |= 1 << prio.unwrap_or(0);
        }

        if mask & 1 == 0 {
            Ok(Self(mask))
        } else {
            Err(anyhow!("Weird char in line {s}"))
        }
    }
}

struct Backpack(Compartment, Compartment);

impl FromStr for Backpack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            return Err(anyhow!("Line does not have even number of items: {s:?}"));
        }

        let (l, r) = s.split_at(s.len() / 2);

        Ok(Self(l.parse()?, r.parse()?))
    }
}

fn parse_input(input: &str) -> Result<Vec<Backpack>> {
    input.lines().map(str::parse).collect()
}

fn part1(bp: &[Backpack]) -> Result<u32> {
    bp.iter().map(|b| b.0.intersection(b.1).single_item()).sum()
}

fn part2(bp: &[Backpack]) -> Result<u32> {
    bp.chunks_exact(3)
        .map(|group| {
            group
                .iter()
                .map(|elf| elf.0.union(elf.1))
                .fold(Compartment(!0), |acc, elf| acc.intersection(elf))
                .single_item()
        })
        .sum()
}

fn main() -> Result<()> {
    let input = read_to_string("input/day03.txt")?;
    let input = parse_input(&input)?;

    let part1 = part1(&input)?;
    println!("Part 1: {part1}");

    let part2 = part2(&input)?;
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let input = parse_input(input).unwrap();

        assert_eq!(part1(&input).unwrap(), 157);
        assert_eq!(part2(&input).unwrap(), 70);
    }
}
