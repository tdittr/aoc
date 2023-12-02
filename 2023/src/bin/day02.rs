use anyhow::{anyhow, bail};
use std::fs::read_to_string;
use std::str::FromStr;

fn part1(games: &[Game]) -> u32 {
    let red = 12;
    let green = 13;
    let blue = 14;

    games
        .into_iter()
        .filter(|g| {
            g.rounds
                .iter()
                .all(|r| r.red <= red && r.green <= green && r.blue <= blue)
        })
        .map(|game| game.id)
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games
        .into_iter()
        .map(|g| g.min_stones())
        .map(|Round { red, green, blue }| red * green * blue)
        .sum()
}

fn main() -> anyhow::Result<()> {
    let input = read_to_string("inputs/day02.txt")?;
    let games = parse_games(&input)?;

    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));

    Ok(())
}

fn parse_games(input: &str) -> anyhow::Result<Vec<Game>> {
    input.lines().map(Game::from_str).collect()
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn min_stones(&self) -> Round {
        self.rounds
            .iter()
            .fold(Round::default(), |acc, round| Round {
                red: acc.red.max(round.red),
                green: acc.green.max(round.green),
                blue: acc.blue.max(round.blue),
            })
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, rounds) = s
            .split_once(':')
            .ok_or_else(|| anyhow!("Weird game {s:?}"))?;
        let start = start.trim_start_matches("Game ");
        let id = start.parse()?;

        let rounds = rounds
            .split(';')
            .map(Round::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { id, rounds })
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r = None;
        let mut g = None;
        let mut b = None;

        for elem in s.split(',') {
            let (num, name) = elem
                .trim()
                .split_once(" ")
                .ok_or_else(|| anyhow!("Could not split element: {elem}"))?;
            let num = num.parse()?;
            let old = match name {
                "red" => r.replace(num),
                "green" => g.replace(num),
                "blue" => b.replace(num),
                other => bail!("Weird color: {other}"),
            };
            if old.is_some() {
                bail!("Double color: {name}");
            }
        }

        Ok(Self {
            red: r.unwrap_or_default(),
            green: g.unwrap_or_default(),
            blue: b.unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn parsing() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = parse_games(input).unwrap();

        assert_eq!(
            games[1],
            Game {
                id: 2,
                rounds: vec![
                    Round {
                        red: 0,
                        green: 2,
                        blue: 1,
                    },
                    Round {
                        red: 1,
                        green: 3,
                        blue: 4,
                    },
                    Round {
                        red: 0,
                        green: 1,
                        blue: 1,
                    }
                ]
            }
        );

        assert_eq!(part1(&games), 8);
    }

    #[rstest]
    #[case("1 red", Round { red: 1, green: 0, blue: 0})]
    #[case("1 red, 2 blue", Round { red: 1, green: 0, blue: 2})]
    #[case("1 green, 22 blue, 123 red", Round { red: 123, green: 1, blue: 22})]
    fn parse_game(#[case] input: &str, #[case] expected: Round) {
        assert_eq!(input.parse::<Round>().unwrap(), expected);
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Round { red: 4, green: 2, blue: 6})]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Round { red: 1, green: 3, blue: 4})]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", Round { red: 20, green: 13, blue: 6})]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", Round { red: 14, green: 3, blue: 15})]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Round { red: 6, green: 3, blue: 2})]
    fn min_stones(#[case] game: &str, #[case] expected: Round) {
        let game: Game = game.parse().unwrap();

        assert_eq!(game.min_stones(), expected);
    }
}
