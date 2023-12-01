#![warn(clippy::pedantic)]

use anyhow::{anyhow, Context, Result};
use ndarray::Array2;
use num::integer::sqrt;
use pathfinding::directed::astar::astar;
use std::fs::read_to_string;

type Input = Map;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Map {
    start: (usize, usize),
    end: (usize, usize),
    heights: Array2<u8>,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut start = None;
    let mut end = None;
    let mut width = None;
    let mut height = 0;

    let map: Vec<_> = input
        .lines()
        .inspect(|&line| {
            width.get_or_insert_with(|| line.chars().count());
            height += 1;
        })
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, height)| ((y, x), height))
        })
        .map(|(coord, height)| {
            Ok(match height {
                'a'..='z' => u8::try_from(height).unwrap() - u8::try_from('a').unwrap(),
                'S' => {
                    start = Some(coord);
                    0
                }
                'E' => {
                    end = Some(coord);
                    25
                }
                _ => return Err(anyhow!("Invalid height: {height:?}")),
            })
        })
        .collect::<Result<_>>()?;

    let heights = Array2::from_shape_vec((height, width.unwrap()), map)?;

    Ok(Map {
        start: start.context("No start found")?,
        end: end.context("No end found")?,
        heights,
    })
}

fn neighbours(
    map: &Array2<u8>,
    (x, y): (usize, usize),
    dir: Dir,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let current_height = map[(x, y)];

    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter_map(move |(d_x, d_y)| {
            let new_x = usize::try_from(isize::try_from(x).unwrap() + d_x).ok()?;
            let new_y = usize::try_from(isize::try_from(y).unwrap() + d_y).ok()?;
            let new_coord = (new_x, new_y);

            let new_height = *map.get(new_coord)?;

            let dist = match dir {
                Dir::Up => new_height.saturating_sub(current_height),
                Dir::Down => current_height.saturating_sub(new_height),
            };
            if dist <= 1 {
                Some(new_coord)
            } else {
                None
            }
        })
}

fn part1(input: &Input) -> usize {
    let path = astar(
        &input.start,
        |coord| {
            neighbours(&input.heights, *coord, Dir::Up)
                .map(|coord| (coord, 1) /* Cost is always 1 */)
        },
        |(x, y)| sqrt(x.pow(2) + y.pow(2)),
        |coord| coord == &input.end,
    )
    .unwrap();

    path.1
}

fn part2(input: &Input) -> usize {
    let path = astar(
        &input.end,
        |coord| {
            neighbours(&input.heights, *coord, Dir::Down)
                .map(|coord| (coord, 1) /* Cost is always 1 */)
        },
        |_| 0, // TODO: replace with nearest non visited 'a'
        |&coord| input.heights[coord] == 0,
    )
    .unwrap();

    path.1
}

fn main() -> Result<()> {
    let input = read_to_string("input/day12.txt").unwrap();
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
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
        let input = parse_input(input).unwrap();
        assert_eq!(part1(&input), 31);
        assert_eq!(part2(&input), 29);
    }
}
