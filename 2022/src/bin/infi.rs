#![warn(clippy::pedantic)]

use anyhow::{anyhow, Context, Result};
use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::str::FromStr;

type Input = Vec<Inst>;

type Coord = (i32, i32);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Inst {
    Turn(i16),
    Walk(i32),
    Jump(i32),
}

impl FromStr for Inst {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(
            match s
                .split_once(' ')
                .with_context(|| format!("malformed line {s}"))?
            {
                ("draai", int) => Self::Turn(int.parse()?),
                ("loop", int) => Self::Walk(int.parse()?),
                ("spring", int) => Self::Jump(int.parse()?),
                _ => return Err(anyhow!("Invalid instruction: {s:?}")),
            },
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}

impl Dir {
    fn turn(self, deg: i16) -> Self {
        assert_eq!(deg % 45, 0);

        let new = (self as i16) + deg / 45;
        let new = new.rem_euclid(8);

        assert!((0..=7).contains(&new));

        Self::from_int(u8::try_from(new).unwrap()).unwrap()
    }

    fn jump(self, oud: Coord, lang: i32) -> Coord {
        let (d_x, d_y) = match self {
            Dir::North => (0, 1),
            Dir::NorthEast => (1, 1),
            Dir::East => (1, 0),
            Dir::SouthEast => (1, -1),
            Dir::South => (0, -1),
            Dir::SouthWest => (-1, -1),
            Dir::West => (-1, 0),
            Dir::NorthWest => (-1, 1),
        };

        (lang * d_x + oud.0, lang * d_y + oud.1)
    }

    fn from_int(dir: u8) -> Option<Self> {
        match dir {
            0 => Some(Self::North),
            1 => Some(Self::NorthEast),
            2 => Some(Self::East),
            3 => Some(Self::SouthEast),
            4 => Some(Self::South),
            5 => Some(Self::SouthWest),
            6 => Some(Self::West),
            7 => Some(Self::NorthWest),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct KerstMens {
    pos: Coord,
    dir: Dir,
}

impl Default for KerstMens {
    fn default() -> Self {
        Self {
            pos: (0, 0),
            dir: Dir::North,
        }
    }
}

impl KerstMens {
    fn exec(&mut self, inst: Inst) {
        match inst {
            Inst::Turn(deg) => self.dir = self.dir.turn(deg),
            Inst::Jump(lang) | Inst::Walk(lang) => self.pos = self.dir.jump(self.pos, lang),
        }
    }
}

fn parse_input(input: &str) -> Result<Input> {
    input.lines().map(str::parse).collect()
}

fn part1(input: &Input) -> i32 {
    let mut km = KerstMens::default();
    for inst in input {
        km.exec(*inst);
    }

    km.pos.0.abs() + km.pos.1.abs()
}

fn part2(input: &Input) -> String {
    let mut traces = BTreeSet::new();
    let mut km = KerstMens::default();
    traces.insert(km.pos);

    for inst in input {
        match *inst {
            inst @ (Inst::Turn(_) | Inst::Jump(_)) => {
                km.exec(inst);
                traces.insert(km.pos);
            }
            Inst::Walk(stapps) => {
                for _ in 0..stapps {
                    km.exec(Inst::Walk(1));
                    traces.insert(km.pos);
                }
            }
        }
    }

    let min = traces.first().unwrap();
    let max = traces.last().unwrap();

    let mut buff = String::new();
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            let c = if traces.contains(&(x, (max.1 - y))) {
                '█'
            } else {
                ' '
            };

            buff.push(c);
        }
        buff.push('\n');
    }

    buff
}

fn main() -> Result<()> {
    let input = read_to_string("input/infi.txt").unwrap();
    let input = parse_input(&input)?;

    let part1 = part1(&input);
    println!("Part 1: {part1}");

    let part2 = part2(&input);
    println!("Part 2:\n{part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = parse_input(
            "draai 90
loop 6
spring 2
draai -45
loop 2",
        )
        .unwrap();

        assert_eq!(part1(&input), 12);
        assert_eq!(
            part2(&input),
            "          █
         █ 
███████ █  
"
            .to_owned()
        );
    }
}
