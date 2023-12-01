#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use itertools::chain;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Item(u8);

impl TryFrom<u8> for Item {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            c @ b'a'..=b'z' => Ok(Self(c - b'a' + 1)),
            c @ b'A'..=b'Z' => Ok(Self(c - b'A' + 27)),
            other => Err(anyhow!("Unexpected item: {other}")),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Rucksack(HashSet<Item>, HashSet<Item>);

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(anyhow!("Input {s:?} is not ASCII"));
        }

        if s.len() % 2 != 0 {
            return Err(anyhow!("Input length of {s:?} is not even"));
        }

        let (a, b) = s.split_at(s.len() / 2);

        let parse_side =
            |side: &str| -> Result<HashSet<_>> { side.bytes().map(Item::try_from).collect() };
        Ok(Self(parse_side(a)?, parse_side(b)?))
    }
}

impl Rucksack {
    fn diff(&self) -> Vec<Item> {
        self.0.intersection(&self.1).copied().collect()
    }

    fn all(&self) -> HashSet<Item> {
        chain!(&self.0, &self.1).copied().collect()
    }
}

fn parse_input(input: &str) -> Result<Vec<Rucksack>> {
    input.lines().map(str::parse).collect()
}

fn part1<'a>(input: impl Iterator<Item = &'a Rucksack>) -> Result<u32> {
    input
        .map(|r| {
            let diff = r.diff();
            if diff.len() != 1 {
                return Err(anyhow!("Diff contains not exactly one item: {diff:?}"));
            }
            Ok(u32::from(diff[0].0))
        })
        .sum()
}

fn part2(input: &[Rucksack]) -> Result<u32> {
    input
        .chunks_exact(3)
        .map(|group| {
            let ab: HashSet<_> = group[0]
                .all()
                .intersection(&group[1].all())
                .copied()
                .collect();
            let item: Vec<_> = ab.intersection(&group[2].all()).copied().collect();

            if item.len() != 1 {
                return Err(anyhow!("Group contains not exactly one badge: {item:?}"));
            }

            Ok(u32::from(item[0].0))
        })
        .sum()
}

fn main() -> Result<()> {
    let input = read_to_string("input/day03.txt")?;
    let input = parse_input(&input)?;

    let part1 = part1(input.iter())?;
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

        assert_eq!(input[0].diff(), vec![Item::try_from(b'p').unwrap()]);

        assert_eq!(part1(input.iter()).unwrap(), 157);
        assert_eq!(part2(&input).unwrap(), 70);
    }
}
