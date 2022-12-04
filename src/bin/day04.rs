use std::fs::read_to_string;
use std::ops::RangeInclusive;

type Group = (RangeInclusive<u32>, RangeInclusive<u32>);

fn parse_range(input: &str) -> RangeInclusive<u32> {
    let (from, to) = input.split_once("-").unwrap();
    from.parse().unwrap()..=to.parse().unwrap()
}

fn parse_input(input: &str) -> Vec<Group> {
    input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(',').unwrap();
            (parse_range(l), parse_range(r))
        })
        .collect()
}

fn overlap_fully(g: &Group) -> bool {
    let contains = |a: &RangeInclusive<u32>, b: &RangeInclusive<u32>| -> bool {
        a.start() <= b.start() && a.end() >= b.end()
    };

    contains(&g.0, &g.1) || contains(&g.1, &g.0)
}

fn overlap_atall(g: &Group) -> bool {
    let contains = |a: &RangeInclusive<u32>, b: &RangeInclusive<u32>| -> bool {
        a.start() <= b.start() && b.end() <= a.start()
            || b.end() >= a.start() && b.start() <= a.start()
    };

    contains(&g.0, &g.1) || contains(&g.1, &g.0)
}

fn part1(g: &[Group]) -> usize {
    g.iter().filter(|&g| overlap_fully(g)).count()
}

fn part2(g: &[Group]) -> usize {
    g.iter().filter(|&g| overlap_atall(g)).count()
}

fn main() {
    let input = read_to_string("input/day04.txt").unwrap();
    let input = parse_input(&input);

    let part1 = part1(&input);
    println!("Part 1: {part1}");

    let part2 = part2(&input);
    println!("Part 2: {part2}");
}
