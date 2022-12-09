#![warn(clippy::pedantic)]

use anyhow::{Context, Result};
use itertools::izip;
use ndarray::{par_azip, Array1, Array2, ArrayView2, Axis};
use std::fs::read_to_string;
use std::str::FromStr;

type Input = Array2<u8>;

fn parse_input(input: &str) -> Result<Input> {
    let len = input.lines().next().context("No lines")?.chars().count();

    let mat: Result<Vec<u8>> = input
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|dig| Ok(u8::from_str(dig.to_string().as_str())?))
        })
        .collect();

    let mat = mat?;

    Array2::from_shape_vec((len, len), mat).context("weird shape")
}

fn seeable_from_up(map: ArrayView2<u8>) -> Array2<bool> {
    let mut max_height: Option<Array1<u8>> = None;
    let mut seeable = Array2::from_elem(map.raw_dim(), false);

    for (heights, mut seeable) in izip!(map.rows(), seeable.rows_mut()) {
        match max_height.as_mut() {
            None => {
                max_height = Some(heights.to_owned());
                seeable.fill(true);
            }
            Some(max_height) => {
                par_azip!((m in max_height, s in &mut seeable, h in &heights) {
                    *s = h > m;
                    *m =  u8::max(*m, *h);
                });
            }
        }
    }
    seeable
}

fn part1(map: ArrayView2<u8>) -> usize {
    // Up
    let mut up = seeable_from_up(map);

    // Down
    let mut g2 = map;
    g2.invert_axis(Axis(0));
    let mut down = seeable_from_up(g2);
    down.invert_axis(Axis(0));

    // Left
    let mut rot = map;
    rot.swap_axes(0, 1);
    let mut left = seeable_from_up(rot);
    left.swap_axes(0, 1);

    // Right
    let mut rot2 = rot;
    rot2.invert_axis(Axis(0));
    let mut right = seeable_from_up(rot2);
    right.invert_axis(Axis(0));
    right.swap_axes(0, 1);

    par_azip!((a in &mut up, b in &down, c in &left, d in &right) *a |= b | c | d);

    up.iter().filter(|e| **e).count()
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn apply(self, mut pos: (usize, usize), map: ArrayView2<u8>) -> Option<(usize, usize)> {
        match self {
            Dir::Up => pos.0 = pos.0.checked_sub(1)?,
            Dir::Down => pos.0 += 1,
            Dir::Left => pos.1 = pos.1.checked_sub(1)?,
            Dir::Right => pos.1 += 1,
        }

        // Check if the pos is in the map
        map.get(pos).map(|_| pos)
    }
}

fn see_trees(map: ArrayView2<u8>, mut pos: (usize, usize), dir: Dir) -> usize {
    let mut can_see = 0;
    let own_height = map[pos];
    loop {
        pos = match dir.apply(pos, map) {
            None => break,
            Some(p) => p,
        };

        can_see += 1;

        if map[pos] >= own_height {
            break;
        }
    }

    can_see
}

fn part2(heights: ArrayView2<u8>) -> Result<usize> {
    let mut score = Array2::<usize>::from_elem(heights.raw_dim(), 1);

    par_azip!((index (i, j), s in &mut score) {
        *s *= see_trees(heights, (i, j), Dir::Up);
        *s *= see_trees(heights, (i, j), Dir::Down);
        *s *= see_trees(heights, (i, j), Dir::Left);
        *s *= see_trees(heights, (i, j), Dir::Right);
    });

    score.iter().max().copied().context("No elements")
}

fn main() -> Result<()> {
    let input = read_to_string("input/day08.txt").unwrap();
    let input = parse_input(&input)?;

    let part1 = part1(input.view());
    println!("Part 1: {part1}");

    let part2 = part2(input.view())?;
    println!("Part 2: {part2}");

    Ok(())
}
