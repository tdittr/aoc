extern crate core;

use anyhow::{anyhow, Context, Result};
use compact_str::CompactString;
use hashbrown::HashMap;
use std::fmt::{Debug, Display};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Line {
    Cmd(Cmd),
    LsOutput(LsOutput),
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Cmd {
    CdRoot,
    CdParent,
    CdDir(CompactString),
    Ls,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum LsOutput {
    DirEntry(CompactString),
    FileEntry(usize, CompactString),
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["$", "cd", "/"] => Self::Cmd(Cmd::CdRoot),
            ["$", "cd", ".."] => Self::Cmd(Cmd::CdParent),
            ["$", "cd", dir] => Self::Cmd(Cmd::CdDir((*dir).into())),
            ["$", "ls"] => Self::Cmd(Cmd::Ls),
            ["dir", dir] => Self::LsOutput(LsOutput::DirEntry((*dir).into())),
            [size, name] => Self::LsOutput(LsOutput::FileEntry(size.parse()?, (*name).into())),
            _ => return Err(anyhow!("Can't parse line: {s:?}")),
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum LineState {
    #[default]
    WaitingForLs,
    InLsOutput,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum BeenHere {
    Yep,
    Nope,
}

type Input = Vec<Line>;

fn parse_input(input: &str) -> Result<Input> {
    input.lines().map(str::parse).collect()
}

fn update_pwd<'a>(pwd: &mut Vec<&'a str>, line_state: &mut LineState, cmd: &'a Cmd) -> Result<()> {
    *line_state = LineState::WaitingForLs;
    match cmd {
        Cmd::CdRoot => pwd.clear(),
        Cmd::CdParent => {
            pwd.pop().ok_or_else(|| anyhow!("Did a `cd ..` in `/`"))?;
        }
        Cmd::CdDir(dir) => pwd.push(dir),
        Cmd::Ls => {
            *line_state = LineState::InLsOutput;
        }
    };

    Ok(())
}

fn part1(g: &Input) -> Result<usize> {
    let sizes = dir_sizes(g)?;

    Ok(sizes.values().filter(|&&s| s <= 100_000).sum())
}

fn dir_sizes(g: &Input) -> Result<HashMap<Vec<&str>, usize>> {
    let mut pwd = vec![];
    let mut state = LineState::default();
    let mut dirs: HashMap<Vec<&str>, BeenHere> = HashMap::new();
    let mut sizes: HashMap<Vec<&str>, usize> = HashMap::new();

    for line in g {
        let ls_line = match (line, state) {
            (Line::Cmd(cmd), _) => {
                update_pwd(&mut pwd, &mut state, cmd)?;
                continue;
            }
            (Line::LsOutput(_), LineState::WaitingForLs) => {
                return Err(anyhow!("Ls output where cmd was expected!"));
            }
            (Line::LsOutput(outp), LineState::InLsOutput) => outp,
        };
        dirs.entry_ref(pwd.as_slice()).insert(BeenHere::Yep);

        match ls_line {
            LsOutput::DirEntry(dir) => {
                pwd.push(dir);
                dirs.entry_ref(pwd.as_slice()).or_insert(BeenHere::Nope);
                pwd.pop().unwrap();
            }
            LsOutput::FileEntry(size, _path) => {
                for len in 0..=pwd.len() {
                    let entry = sizes.entry_ref(&pwd[..len]).or_insert(0);
                    *entry += size;
                }
            }
        }
    }

    if dirs.values().any(|visit| *visit == BeenHere::Nope) {
        return Err(anyhow!("Missed a dir"));
    }

    Ok(sizes)
}

fn part2(g: &Input) -> Result<usize> {
    let sizes = dir_sizes(g)?;

    let free_space = 70_000_000 - sizes[&vec![]];
    let min_size = 30000000 - free_space;

    sizes
        .iter()
        .map(|(_, s)| *s)
        .filter(|&s| s >= min_size)
        .min()
        .with_context(|| "no dir with enough size")
}

fn main() -> Result<()> {
    let input = read_to_string("input/day07.txt").unwrap();
    let input = parse_input(&input)?;

    let part1 = part1(&input)?;
    println!("Part 1: {part1}");

    let part2 = part2(&input)?;
    println!("Part 2: {part2:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

        let input = parse_input(input).unwrap();
        assert_eq!(part1(&input).unwrap(), 95437);
        assert_eq!(part2(&input).unwrap(), 24933642);
    }
}
