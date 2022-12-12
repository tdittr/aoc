#![warn(clippy::pedantic)]

use anyhow::{anyhow, Context, Result};
use std::fs::read_to_string;
use std::str::FromStr;

type Input = Vec<Instruction>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    Nop,
    AddX(i64),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        match (tokens.next(), tokens.next()) {
            (Some("noop"), None) => Ok(Self::Nop),
            (Some("addx"), Some(val)) => Ok(Self::AddX(val.parse()?)),
            _ => Err(anyhow!("Invalid line: {s:?}")),
        }
    }
}

fn parse_input(input: &str) -> Result<Input> {
    input.lines().map(str::parse).collect()
}

#[derive(Debug, Eq, PartialEq)]
struct McMachine {
    reg_x: i64,
    waiting: Option<(u8, Instruction)>,
    ip: usize,
    instructions: Vec<Instruction>,
}

impl Default for McMachine {
    fn default() -> Self {
        Self {
            reg_x: 1,
            waiting: None,
            ip: 0,
            instructions: vec![],
        }
    }
}

impl McMachine {
    fn with_instructions(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            ..Default::default()
        }
    }

    fn step(&mut self) -> Result<()> {
        if let Some((to_wait, inst)) = self.waiting.as_mut() {
            *to_wait = to_wait.saturating_sub(1);
            if *to_wait > 0 {
                return Ok(());
            }

            match inst {
                Instruction::Nop => unreachable!(),
                Instruction::AddX(val) => self.reg_x += *val,
            }

            self.waiting = None;
            self.ip += 1;
            return Ok(());
        }

        match self
            .instructions
            .get(self.ip)
            .context("Fell of the program")?
        {
            Instruction::Nop => {
                self.ip += 1;
            }
            inst @ Instruction::AddX(_) => {
                self.waiting = Some((1, *inst));
            }
        }

        Ok(())
    }
}

fn run_for(prog: &Input, steps: usize) -> Result<Vec<i64>> {
    let mut m = McMachine::with_instructions(prog.clone());

    (0..steps)
        .map(|_| {
            m.step()?;
            Ok(m.reg_x)
        })
        .collect()
}

fn part1(prog: &Input) -> Result<i64> {
    let vals = run_for(prog, 221)?;

    Ok([20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|idx| idx as i64 * vals[idx - 2])
        .sum())
}

fn part2(input: &Input) -> Result<String> {
    let mut m = McMachine::with_instructions(input.clone());
    let mut r = String::with_capacity(41 * 6);

    for _y in 0..6 {
        for x in 0..40 {
            if x >= m.reg_x - 1 && x <= m.reg_x + 1 {
                r.push('█');
            } else {
                r.push(' ');
            }

            m.step()?;
        }
        r.push('\n');
    }

    Ok(r)
}

fn main() -> Result<()> {
    let input = read_to_string("input/day10.txt").unwrap();
    let input = parse_input(&input)?;

    let part1 = part1(&input)?;
    println!("Part 1: {part1}");

    let part2 = part2(&input)?;
    println!("Part 2:\n{part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_exmaple() {
        let inp = parse_input(
            "noop
addx 3
addx -5
",
        )
        .unwrap();

        let mut m = McMachine::with_instructions(inp.clone());

        assert_eq!(m.reg_x, 1);
        m.step().unwrap(); // 1

        assert_eq!(m.reg_x, 1);
        m.step().unwrap(); // 2

        assert_eq!(m.reg_x, 1);
        m.step().unwrap(); // 3

        assert_eq!(m.reg_x, 4);
        m.step().unwrap(); // 4

        assert_eq!(m.reg_x, 4);
        m.step().unwrap(); // 5

        assert_eq!(m.reg_x, -1);
        m.step().unwrap_err(); // 6 // done!

        let vals = run_for(&inp, 5).unwrap();
        assert_eq!(vals, vec![1, 1, 4, 4, -1]);
    }

    #[test]
    fn example() {
        let input = parse_input(EXAMPLE).unwrap();

        let vals = run_for(&input, 220).unwrap();
        assert_eq!(vals[20 - 2], 21);
        assert_eq!(vals[220 - 2], 18);

        assert_eq!(
            [20, 60, 100, 140, 180, 220]
                .into_iter()
                .map(|idx| idx as i64 * vals[idx - 2])
                .collect::<Vec<_>>(),
            vec![420, 1140, 1800, 2940, 2880, 3960]
        );

        assert_eq!(part1(&input).unwrap(), 13140);
        assert_eq!(
            part2(&input).unwrap(),
            "██  ██  ██  ██  ██  ██  ██  ██  ██  ██  
███   ███   ███   ███   ███   ███   ███ 
████    ████    ████    ████    ████    
█████     █████     █████     █████     
██████      ██████      ██████      ████
███████       ███████       ███████     
"
        );
    }

    const EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
}
