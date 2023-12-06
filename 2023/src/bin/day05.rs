use anyhow::{bail, Context};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Range {
    src_start: u32,
    dst_start: u32,
    len: u32,
}

impl Range {
    fn try_map(self, src: u32) -> Option<u32> {
        let offset = src.checked_sub(self.src_start)?;
        if offset >= self.len {
            return None;
        }

        Some(self.dst_start + offset)
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<_> = s
            .split_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            dst_start: nums[0],
            src_start: nums[1],
            len: nums[2],
        })
    }
}

#[derive(Debug, Clone)]
struct Map {
    from: String,
    to: String,
    range_map: Vec<Range>,
}

impl Map {
    fn map(&self, src: u32) -> u32 {
        let idx = self
            .range_map
            .binary_search_by_key(&src, |r| r.src_start)
            .unwrap_or_else(|e| e.saturating_sub(1));

        self.range_map[idx].try_map(src).unwrap_or(src)
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = lines.next().context("no header")?;
        let header_parts = header.split(&['-', ' ']).collect::<Vec<_>>();
        let [from, "to", to, "map:"] = header_parts.as_slice() else {
            bail!("header kapott: {header:?}");
        };
        let mut range_map = lines
            .map(str::parse::<Range>)
            .collect::<Result<Vec<_>, _>>()?;

        range_map.sort_by_key(|r| r.src_start);

        Ok(Self {
            from: from.to_string(),
            to: to.to_string(),
            range_map,
        })
    }
}

#[derive(Debug, Clone)]
struct Input {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl Input {
    fn location(&self, seed: u32) -> u32 {
        let mut current_type = "seed".to_string();
        let mut val = seed;
        for map in &self.maps {
            assert_eq!(map.from, current_type);
            val = map.map(val);
            current_type = map.to.clone();
        }

        val
    }

    fn seed_ranges(&self) -> impl Iterator<Item = std::ops::Range<u32>> + '_ {
        assert_eq!(self.seeds.len() % 2, 0);
        self.seeds.chunks_exact(2).map(|ch| ch[0]..(ch[0] + ch[1]))
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s.split("\n\n");

        let seeds = blocks
            .next()
            .context("no seeds")?
            .split_whitespace()
            .skip(1)
            .map(str::parse::<u32>)
            .collect::<Result<_, _>>()?;
        let maps = blocks.map(str::parse::<Map>).collect::<Result<_, _>>()?;

        Ok(Self { seeds, maps })
    }
}

fn main() {
    let input = Input::from_str(read_to_string("inputs/day05.txt").unwrap().as_str()).unwrap();

    let p1 = input
        .seeds
        .iter()
        .map(|seed| input.location(*seed))
        .min()
        .unwrap();
    println!("Part 1: {p1}");

    let p2 = input
        .seed_ranges()
        .flatten()
        .map(|seed| input.location(seed))
        .min()
        .unwrap();
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn parse() {
        let parsed: Input = EXAMPLE.parse().unwrap();

        assert_eq!(parsed.seeds, vec![79, 14, 55, 13]);
        assert_eq!(parsed.maps[0].from, "seed");
        assert_eq!(parsed.maps[0].to, "soil");
        assert_eq!(parsed.maps[0].range_map[1].dst_start, 50);
        assert_eq!(parsed.maps[0].range_map[1].src_start, 98);
        assert_eq!(parsed.maps[0].range_map[1].len, 2);

        assert_eq!(parsed.maps[0].map(79), 81);
        assert_eq!(parsed.maps[0].map(14), 14);
        assert_eq!(parsed.maps[0].map(55), 57);
        assert_eq!(parsed.maps[0].map(13), 13);

        assert_eq!(parsed.maps[2].map(53), 49);

        assert_eq!(parsed.location(79), 82);

        assert_eq!(parsed.maps[0].map(14), 14);
        assert_eq!(parsed.maps[1].map(14), 53);
        assert_eq!(parsed.maps[2].map(53), 49);
        assert_eq!(parsed.maps[3].map(49), 42);
        assert_eq!(parsed.maps[4].map(42), 42);
        assert_eq!(parsed.maps[5].map(42), 43);
        assert_eq!(parsed.maps[6].map(43), 43);

        assert_eq!(parsed.location(55), 86);
        assert_eq!(parsed.location(13), 35);
    }
}
