use anyhow::{Context, Error};
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    pub fn min_hold(&self) -> Option<u64> {
        for hold in 0..=self.time {
            let remain = self.time - hold;

            let mut dist = 0;
            for t in 0..remain {
                dist += hold;
            }

            if dist > self.dist {
                return Some(hold);
            }
        }

        None
    }

    pub fn max_hold(&self) -> Option<u64> {
        for hold in (0..=self.time).rev() {
            let remain = self.time - hold;

            let mut dist = 0;
            for t in 0..remain {
                dist += hold;
            }

            if dist > self.dist {
                return Some(hold);
            }
        }

        None
    }

    pub fn num_holds(&self) -> u64 {
        let a = self.min_hold().unwrap();
        let b = self.max_hold().unwrap();

        (a..=b).count() as u64
    }
}

struct Input(Vec<Race>);

impl Input {
    pub fn p1(&self) -> u64 {
        self.0.iter().map(Race::num_holds).product()
    }

    pub fn into_p2(&self) -> Race {
        let time = self
            .0
            .iter()
            .map(|r| r.time.to_string())
            .collect::<Vec<_>>()
            .concat()
            .parse()
            .unwrap();
        let dist = self
            .0
            .iter()
            .map(|r| r.dist.to_string())
            .collect::<Vec<_>>()
            .concat()
            .parse()
            .unwrap();

        Race { time, dist }
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let times: Result<Vec<_>, _> = lines
            .next()
            .context("")?
            .split_whitespace()
            .skip(1)
            .map(u64::from_str)
            .collect();
        let distances: Result<Vec<_>, _> = lines
            .next()
            .context("")?
            .split_whitespace()
            .skip(1)
            .map(u64::from_str)
            .collect();
        assert!(lines.next().is_none());

        let races = times?
            .iter()
            .zip(distances?)
            .map(|(&time, dist)| Race { time, dist })
            .collect();

        Ok(Self(races))
    }
}

fn main() -> Result<(), Error> {
    let input = Input::from_str(read_to_string("inputs/day06.txt")?.as_str())?;

    println!("P1: {}", input.p1());
    println!("P2: {}", input.into_p2().num_holds());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = Input::from_str(
            "Time:      7  15   30
Distance:  9  40  200",
        )
        .unwrap();

        let r = input.0.clone();

        assert_eq!(r.len(), 3);
        assert_eq!(
            r[2],
            Race {
                time: 30,
                dist: 200
            }
        );
        assert_eq!(r[0].min_hold(), Some(2));
        assert_eq!(r[0].max_hold(), Some(5));
        assert_eq!(r[0].num_holds(), 4);
        assert_eq!(r[1].num_holds(), 8);
        assert_eq!(r[2].num_holds(), 9);
        assert_eq!(input.p1(), 288);
    }
}
