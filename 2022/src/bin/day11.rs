#![warn(clippy::pedantic)]

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::fs::read_to_string;
use std::mem;
use std::str::FromStr;

type Input = Vec<RefCell<Monkey>>;

type Item = u64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
enum Operation {
    Mul(Item),
    Add(Item),
    Square,
}

impl Operation {
    fn apply(self, old: Item) -> Item {
        match self {
            Operation::Mul(x) => old * x,
            Operation::Add(x) => old + x,
            Operation::Square => old * old,
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["new", "=", "old", "*", "old"] => Ok(Self::Square),
            ["new", "=", "old", "*", int] => Ok(Self::Mul(int.parse()?)),
            ["new", "=", "old", "+", int] => Ok(Self::Add(int.parse()?)),
            _ => Err(anyhow!("Invalid operation: {s:?}")),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct Monkey {
    items: Vec<Item>,
    op: Operation,
    divides_by: Item,
    on_true_throw_to: usize,
    on_false_throw_to: usize,
    inspect_count: usize,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let monkey_to_short = || format!("Monkey to short: {s:?}");
        let mut lines = s.lines();

        let monkey_id = lines
            .next()
            .context("can not parse empty string to monkey")?;
        if !(monkey_id.starts_with("Monkey ") || monkey_id.ends_with(':')) {
            return Err(anyhow!("invalid monkey introduction: {monkey_id:?}"));
        }

        let start_items = lines.next().context("monkey to short")?;
        let items = start_items
            .strip_prefix("  Starting items: ")
            .context("unexpected start of starting_items")?
            .split_terminator(',')
            .map(|item| Ok(item.trim().parse()?))
            .collect::<Result<Vec<Item>>>()?;

        let op = lines
            .next()
            .context("monkey to short")?
            .strip_prefix("  Operation: ")
            .context("monkey missing op")?
            .trim()
            .parse()?;

        let divides_by = lines
            .next()
            .with_context(monkey_to_short)?
            .strip_prefix("  Test: divisible by ")
            .context("monkey missing test")?
            .trim()
            .parse()?;

        let on_true_throw_to = lines
            .next()
            .with_context(monkey_to_short)?
            .strip_prefix("    If true: throw to monkey ")
            .context("monkey missing on true")?
            .trim()
            .parse()?;

        let on_false_throw_to = lines
            .next()
            .with_context(monkey_to_short)?
            .strip_prefix("    If false: throw to monkey ")
            .context("monkey missing on false")?
            .trim()
            .parse()?;

        if let Some(extra) = lines.next() {
            return Err(anyhow!("Monkey contains extra data: {extra:?}"));
        }

        Ok(Self {
            items,
            op,
            divides_by,
            on_true_throw_to,
            on_false_throw_to,
            inspect_count: 0,
        })
    }
}

fn find_mod(monkeys: &[RefCell<Monkey>]) -> Item {
    let divisors = monkeys.iter().map(|m| m.borrow().divides_by).product();

    // TODO: Optimize to only needed primes?
    divisors
}

fn round(monkeys: &mut [RefCell<Monkey>], relief: bool, modulo: Item) {
    let monkeys = &*monkeys;

    for monkey in monkeys {
        let mut monkey = monkey.borrow_mut();
        let items = mem::take(&mut monkey.items);
        for mut item in items {
            // Monkey starts inspecting
            item = monkey.op.apply(item);
            monkey.inspect_count += 1;

            if relief {
                // Be relieved
                item /= 3;
            }

            // Reduce numbers
            item %= modulo;

            // Perform test
            let throw_to = if item % monkey.divides_by == 0 {
                monkey.on_true_throw_to
            } else {
                monkey.on_false_throw_to
            };

            // Throw item
            monkeys[throw_to].borrow_mut().items.push(item);
        }
    }
}

fn parse_input(input: &str) -> Result<Input> {
    input
        .split("\n\n")
        .map(|m| Ok(RefCell::new(m.trim().parse()?)))
        .collect()
}

fn part1(input: &Input) -> usize {
    let mut input = input.clone();
    let modulo = find_mod(&input);
    for _ in 1..=20 {
        round(&mut input, true, modulo);
    }

    let mut inspections: Vec<_> = input.iter().map(|m| m.borrow().inspect_count).collect();
    inspections.sort_by_key(|cnt| Reverse(*cnt));
    inspections[0] * inspections[1]
}

fn part2(input: &Input) -> usize {
    let mut input = input.clone();
    let modulo = find_mod(&input);

    dbg!(modulo);

    for _ in 1..=10_000 {
        round(&mut input, false, modulo);
    }

    let mut inspections: Vec<_> = input.iter().map(|m| m.borrow().inspect_count).collect();
    inspections.sort_by_key(|cnt| Reverse(*cnt));
    inspections[0] * inspections[1]
}

fn main() -> Result<()> {
    let input = read_to_string("input/day11.txt").unwrap();
    let input = parse_input(&input)?;

    println!("{}", serde_json::to_string_pretty(&input)?);

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
    fn parsing() {
        let input = parse_input(EXAMPLE).unwrap();
        let m = input.get(0).map(|m| m.borrow().clone()).unwrap();
        assert_eq!(
            m,
            Monkey {
                items: vec![79, 98],
                op: Operation::Mul(19),
                divides_by: 23,
                on_true_throw_to: 2,
                on_false_throw_to: 3,
                inspect_count: 0,
            }
        );
    }

    #[test]
    fn example() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 10_605);
        assert_eq!(part2(&input), 2_713_310_158);
    }

    #[test]
    fn step_example() {
        let mut input = parse_input(EXAMPLE).unwrap();
        let modulo = find_mod(&input);

        // Round 1
        round(&mut input, true, modulo);
        let items: Vec<Vec<Item>> = input.iter().map(|m| m.borrow().items.clone()).collect();
        assert_eq!(
            items,
            vec![
                vec![20, 23, 27, 26],
                vec![2080, 25, 167, 207, 401, 1046],
                vec![],
                vec![]
            ]
        );

        // Round 2
        round(&mut input, true, modulo);
        let items: Vec<Vec<Item>> = input.iter().map(|m| m.borrow().items.clone()).collect();
        assert_eq!(
            items,
            vec![
                vec![695, 10, 71, 135, 350],
                vec![43, 49, 58, 55, 362],
                vec![],
                vec![]
            ]
        );

        for _ in 3..=20 {
            round(&mut input, true, modulo);
        }
        let items: Vec<Vec<Item>> = input.iter().map(|m| m.borrow().items.clone()).collect();
        assert_eq!(
            items,
            vec![
                vec![10, 12, 14, 26, 34],
                vec![245, 93, 53, 199, 115],
                vec![],
                vec![]
            ]
        );

        let inspections: Vec<_> = input.iter().map(|m| m.borrow().inspect_count).collect();
        assert_eq!(inspections, vec![101, 95, 7, 105]);
    }

    pub const EXAMPLE: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
}
