use std::fs::read_to_string;

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find(char::is_ascii_digit).unwrap();
            let last = line.chars().rfind(char::is_ascii_digit).unwrap();

            let num = format!("{first}{last}");
            num.parse::<usize>().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input.lines().map(part2_line).sum()
}

fn part2_line(line: &str) -> usize {
    let digits = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];

    let first = find_any(digits, line, Dir::Forward).unwrap();
    let first = first % 10;

    let last = find_any(digits, line, Dir::Reverse).unwrap();
    let last = last % 10;

    let num = format!("{first}{last}");
    num.parse::<usize>().unwrap()
}

enum Dir {
    Forward,
    Reverse,
}

fn find_any(needles: [&str; 20], mut haystack: &str, dir: Dir) -> Option<usize> {
    loop {
        for (idx, d) in needles.iter().enumerate() {
            match dir {
                Dir::Forward if haystack.starts_with(d) => return Some(idx),
                Dir::Reverse if haystack.ends_with(d) => return Some(idx),
                _ => continue,
            }
        }
        haystack = match dir {
            Dir::Forward => &haystack.get(1..)?,
            Dir::Reverse => &haystack.get(..haystack.len() - 1)?,
        };
    }
}

fn main() {
    let input = read_to_string("inputs/day01.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

        assert_eq!(part1(input), 142);
    }

    #[test]
    fn t_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

        assert_eq!(part2_line("two1nine"), 29);
        assert_eq!(part2(input), 281);
    }
}
