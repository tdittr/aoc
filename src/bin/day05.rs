#![warn(clippy::pedantic)]

use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use scan_fmt::scan_fmt;

use std::fs::read_to_string;

use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (amount, from, to) = scan_fmt!(s, "move {d} from {d} to {d}", usize, usize, usize)
            .with_context(|| format!("While parsing {s}"))?;
        Ok(Self { amount, from, to })
    }
}

type Stack = Vec<char>;
type Input = (Vec<Stack>, Vec<Move>);

fn parse_stack_line(line: &str) -> impl Iterator<Item = Option<&str>> {
    static LINE_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"(?x)
                \s? # Leading space (missing for first group)
                (?:
                    (?:\[(?P<box>.+?)\]) # Box
                    |
                    (?:\s\s\s) # Air
                )",
        )
        .expect("Dev did not make an error when writing regex...")
    });

    LINE_REGEX
        .captures_iter(line)
        .map(|cap| cap.name("box").map(|cap| cap.as_str()))
}

fn parse_stacks(input: &str) -> Result<Vec<Stack>> {
    let max_height = input.lines().count();
    let mut lines = input.lines().rev();
    let indecies = lines.next().context("Stacks were empty")?;
    let mut stacks: Vec<Stack> = indecies
        .split_whitespace()
        .enumerate()
        .map(|(pos, idx)| {
            let idx = usize::from_str(idx)
                .with_context(|| format!("Stack index {idx:?} is not an int"))?;

            if pos + 1 != idx {
                return Err(anyhow!("Indecies do not match 1,2,3,.."));
            }

            Ok(Vec::with_capacity(max_height))
        })
        .collect::<Result<_>>()?;

    stacks.insert(0, vec![]); // Add an empty stack in the front to avoid idx to col calculations
    let max_len = stacks.len();

    for l in lines {
        for (col, r#box) in parse_stack_line(l)
            .enumerate()
            .filter_map(|(col, b)| b.map(|b| (col, b)))
        {
            let idx = col + 1;
            let stack = stacks.get_mut(idx).with_context(|| {
                format!("Unexpected number of columns of boxes... expected max {max_len} got {col}")
            })?;

            if r#box.chars().count() != 1 {
                return Err(anyhow!("Box contains not exactly one item: {:?}", r#box));
            }

            stack.push(r#box.chars().next().unwrap());
        }
    }

    Ok(stacks)
}

fn parse_input(input: &str) -> Result<Input> {
    let (stacks, moves) = input
        .split_once("\n\n")
        .context("Input is missing segment seperator...")?;

    Ok((
        parse_stacks(stacks)?,
        moves.lines().map(str::parse).collect::<Result<_>>()?,
    ))
}

fn get_both<T>(sli: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    assert_ne!(a, b);

    if a < b {
        let (front, back) = sli.split_at_mut(b);
        (&mut front[a], &mut back[0])
    } else {
        let (front, back) = sli.split_at_mut(a);
        (&mut back[0], &mut front[b])
    }
}

fn apply_moves((stacks, moves): &Input, pickup_multiple: bool) -> String {
    let mut stacks = stacks.clone();

    for m in moves {
        let (from, to) = get_both(&mut stacks, m.from, m.to);
        let start = from.len() - m.amount;
        if pickup_multiple {
            to.extend(from.drain(start..));
        } else {
            to.extend(from.drain(start..).rev());
        }
    }

    stacks.iter().filter_map(|s| s.last()).collect()
}

fn part1(input: &Input) -> String {
    apply_moves(input, false)
}

fn part2(input: &Input) -> String {
    apply_moves(input, true)
}

fn main() -> Result<()> {
    let input = read_to_string("input/day05.txt").unwrap();
    let input = parse_input(&input)?;

    let part1 = part1(&input);
    println!("Part 1: {part1}");

    let part2 = part2(&input);
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn regex() {
        let test = "[A] [B]     [C]";
        assert_eq!(
            parse_stack_line(test).collect::<Vec<_>>(),
            vec![Some("A"), Some("B"), None, Some("C")]
        );
    }

    #[test]
    fn regex_emoji() {
        let test = "[🏳️‍🌈] [👨‍👩‍👦‍👦]     [C]";
        assert_eq!(
            parse_stack_line(test).collect::<Vec<_>>(),
            vec![Some("🏳️‍🌈"), Some("👨‍👩‍👦‍👦"), None, Some("C")]
        );
    }

    #[test]
    fn parsing() {
        let (stacks, moves) = parse_input(INPUT).unwrap();

        assert_eq!(stacks[1], "ZN".chars().collect::<Vec<_>>());
        assert_eq!(stacks[2], "MCD".chars().collect::<Vec<_>>());
        assert_eq!(stacks[3], "P".chars().collect::<Vec<_>>());

        assert_eq!(
            moves[0],
            Move {
                amount: 1,
                from: 2,
                to: 1,
            }
        );
        assert_eq!(
            moves[2],
            Move {
                amount: 2,
                from: 2,
                to: 1,
            }
        );
    }

    #[test]
    fn example() {
        let input = parse_input(INPUT).unwrap();
        assert_eq!(part1(&input), "CMZ".to_string());
        assert_eq!(part2(&input), "MCD".to_string());
    }

    #[test]
    fn double_mut() {
        let mut t = vec![0, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(get_both(&mut t, 0, 1), (&mut 0, &mut 1));
        assert_eq!(get_both(&mut t, 3, 7), (&mut 3, &mut 7));
        assert_eq!(get_both(&mut t, 7, 3), (&mut 7, &mut 3));
    }
}
