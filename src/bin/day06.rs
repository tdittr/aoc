#![warn(clippy::pedantic)]

use anyhow::{Context, Result};
use std::fs::read_to_string;

type Input = Vec<u8>;

fn is_uniq(bytes: &[u8]) -> bool {
    // bytes.iter().all_unique() // is nice but uses a HashSet

    bytes
        .iter()
        .enumerate()
        .flat_map(|(idx, &me)| bytes[idx + 1..].iter().map(move |&other| (me, other)))
        .all(|(me, other)| me != other)
}

fn pos_after_n_uniq(g: &Input, n: usize) -> Result<usize> {
    g.windows(n)
        .position(is_uniq)
        .map(|pos| pos + n)
        .context("No unique sequence found")
}

fn part1(g: &Input) -> Result<usize> {
    pos_after_n_uniq(g, 4)
}

fn part2(g: &Input) -> Result<usize> {
    pos_after_n_uniq(g, 14)
}

fn main() -> Result<()> {
    let input = read_to_string("input/day06.txt").unwrap();
    let input = input.into_bytes();

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
    fn example_1() {
        let examples = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (bytes, result) in examples {
            assert_eq!(part1(&Vec::from(bytes)).unwrap(), result);
        }
    }

    #[test]
    fn example_2() {
        let examples = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (bytes, result) in examples {
            assert_eq!(part2(&Vec::from(bytes)).unwrap(), result);
        }
    }
}
