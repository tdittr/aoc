#![warn(clippy::pedantic)]

use anyhow::{anyhow, Context, Result};
use hashbrown::HashSet;
use std::fs::read_to_string;

type Input = Vec<(Dir, usize)>;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn apply(self, mut pos: &mut (isize, isize)) {
        match self {
            Dir::Up => pos.0 -= 1,
            Dir::Down => pos.0 += 1,
            Dir::Left => pos.1 -= 1,
            Dir::Right => pos.1 += 1,
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => return Err(anyhow!("Illegal move dir {s:?}")),
        })
    }
}

fn parse_input(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|l| {
            let (dir, steps) = l.split_once(char::is_whitespace).context("Invalid line")?;
            Ok((Dir::from_str(dir)?, steps.parse()?))
        })
        .collect()
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
struct State {
    head: (isize, isize),
    tail: (isize, isize),
}

impl State {
    fn move_head(&mut self, dir: Dir) {
        dir.apply(&mut self.head);
        self.update_tail();
    }

    fn update_tail(&mut self) {
        #[allow(clippy::unnested_or_patterns)] // Better readable this way
        match (self.head.0 - self.tail.0, self.head.1 - self.tail.1) {
            (-1 | 0 | 1, -1 | 0 | 1) => (),
            (0, 2) => self.tail.1 += 1,
            (2, 0) => self.tail.0 += 1,
            (0, -2) => self.tail.1 -= 1,
            (-2, 0) => self.tail.0 -= 1,

            (1, 2) | (2, 1) | (2, 2) => {
                self.tail.0 += 1;
                self.tail.1 += 1;
            }
            (-1, 2) | (-2, 1) | (-2, 2) => {
                self.tail.0 -= 1;
                self.tail.1 += 1;
            }
            (1, -2) | (2, -1) | (2, -2) => {
                self.tail.0 += 1;
                self.tail.1 -= 1;
            }
            (-1, -2) | (-2, -1) | (-2, -2) => {
                self.tail.0 -= 1;
                self.tail.1 -= 1;
            }
            other => unreachable!("Trying to move {other:?}"),
        }
    }
}

fn part1(g: &Input) -> usize {
    let mut state = State::default();
    let mut visited = HashSet::new();

    for (d, cnt) in g.iter().copied() {
        for _ in 0..cnt {
            state.move_head(d);
            visited.insert(state.tail);
        }
    }

    visited.len()
}

fn part2(g: &Input) -> usize {
    let mut states = [State::default(); 9];
    let mut visited = HashSet::new();

    for (d, cnt) in g.iter().copied() {
        for _ in 0..cnt {
            states[0].move_head(d);
            for i in 1..states.len() {
                states[i].head = states[i - 1].tail;
                states[i].update_tail();
            }
            visited.insert(states[8].tail);
        }
    }

    visited.len()
}

fn main() -> Result<()> {
    let input = read_to_string("input/day09.txt").unwrap();
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

    #[test]
    fn example() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
        let input = parse_input(input).unwrap();

        assert_eq!(part1(&input), 13);
        assert_eq!(part2(&input), 1);
    }
    #[test]
    fn example_p2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        let input = parse_input(input).unwrap();

        assert_eq!(part2(&input), 36);
    }
}
