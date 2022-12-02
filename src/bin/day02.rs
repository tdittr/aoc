#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::fs::read_to_string;
use Outcome::{Draw, Lose, Win};
use Rps::{Paper, Rock, Scissors};

trait FromXyz
where
    Self: Sized,
{
    fn from_xyz(input: &str) -> Result<Self>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    #[cfg(test)]
    fn inverse(self) -> Self {
        match self {
            Win => Lose,
            Draw => Draw,
            Lose => Win,
        }
    }

    fn score(self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

impl FromXyz for Outcome {
    fn from_xyz(input: &str) -> Result<Self> {
        match input {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            other => Err(anyhow!("Unexpected {other} where XYZ was expected")),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    const ALL: [Self; 3] = [Rock, Paper, Scissors];

    fn play(self, other: Self) -> Outcome {
        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Win,
            (a, b) if a == b => Draw,
            _ => Lose,
        }
    }

    fn score_game(self, other: Self) -> u32 {
        self.score() + self.play(other).score()
    }

    fn score(self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn choose_to_get_outcome(self, outcome: Outcome) -> Self {
        // This might look like I was lazy... and that's because I was
        for choice in Self::ALL {
            if choice.play(self) == outcome {
                return choice;
            }
        }

        unreachable!("There should always be a way to get any outcome")
    }

    fn from_abc(input: &str) -> Result<Self> {
        match input {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            other => Err(anyhow!("Unexpected {other} where ABC was expected")),
        }
    }
}

impl FromXyz for Rps {
    fn from_xyz(input: &str) -> Result<Self> {
        match input {
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            other => Err(anyhow!("Unexpected {other} where XYZ was expected")),
        }
    }
}

fn parse_line<T: FromXyz>(line: &str) -> Result<(Rps, T)> {
    let (a, b) = line
        .split_once(' ')
        .ok_or_else(|| anyhow!("Weird line: {line}"))?;

    Ok((Rps::from_abc(a)?, T::from_xyz(b)?))
}

fn parse_input<T: FromXyz>(input: &str) -> Result<Vec<(Rps, T)>> {
    input.trim().lines().map(parse_line).collect()
}

fn part1(games: &[(Rps, Rps)]) -> u32 {
    games
        .iter()
        .copied()
        .map(|(elf, santa)| santa.score_game(elf))
        .sum()
}

fn part2(games: &[(Rps, Outcome)]) -> u32 {
    games
        .iter()
        .copied()
        .map(|(elf, outcome)| (elf, elf.choose_to_get_outcome(outcome)))
        .map(|(elf, santa)| santa.score_game(elf))
        .sum()
}

fn main() -> Result<()> {
    let input = read_to_string("input/day02.txt")?;

    let games1 = parse_input(&input)?;
    let part1 = part1(&games1);
    println!("Part 1: {part1}");

    let games2 = parse_input(&input)?;
    let part2 = part2(&games2);
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::iproduct;

    const INPUT: &str = r"A Y
B X
C Z
";

    #[test]
    fn game_logic_is_consistent() {
        for (a, b) in iproduct!(Rps::ALL, Rps::ALL) {
            assert_eq!(a.play(b), b.play(a).inverse());
        }
    }

    #[test]
    fn example1() {
        let games = parse_input(INPUT).unwrap();
        let score = part1(&games);

        assert_eq!(score, 15);
    }

    #[test]
    fn example2() {
        let games = parse_input(INPUT).unwrap();
        let score = part2(&games);

        assert_eq!(score, 12);
    }
}
