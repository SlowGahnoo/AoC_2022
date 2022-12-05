use std::fs;
use std::{cmp::Ordering, convert::TryFrom};

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone, Copy)]
enum RPC {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<&str> for RPC {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(RPC::Rock),
            "B" | "Y" => Ok(RPC::Paper),
            "C" | "Z" => Ok(RPC::Scissors),
            _ => Err(()),
        }
    }
}

impl Ord for RPC {
    fn cmp(&self, other: &Self) -> Ordering {
        let p1 = *self as i32;
        let p2 = *other as i32;
        if p1 % 3 == (p2 - 1) {
            return Ordering::Less;
        } else if p1 == p2 {
            return Ordering::Equal;
        }
        Ordering::Greater
    }
}

fn get_score(play1: &RPC, play2: &RPC) -> i32 {
    match play1.cmp(play2) {
        Ordering::Less => *play2 as i32 + 6,
        Ordering::Greater => *play2 as i32 + 0,
        Ordering::Equal => *play2 as i32 + 3,
    }
}

fn part1(input: &str) -> i32 {
    let mut score = 0;
    input.lines().for_each(|play| {
        let [p1, p2] =
            <[&str; 2]>::try_from(play.split_whitespace().take(2).collect::<Vec<&str>>()).unwrap();
        let play1 = <RPC>::try_from(p1).unwrap();
        let play2 = <RPC>::try_from(p2).unwrap();
        score += get_score(&play1, &play2);
    });
    score
}

fn part2(input: &str) -> i32 {
    let mut score = 0;
    input.lines().for_each(|play| {
        let [p1, p2] =
            <[&str; 2]>::try_from(play.split_whitespace().take(2).collect::<Vec<&str>>()).unwrap();
        let play1 = <RPC>::try_from(p1).unwrap();
        let mut play2 = <RPC>::try_from(p2).unwrap();
        match p2 {
            "X" => {
                play2 = match play1 {
                    RPC::Rock => RPC::Scissors,
                    RPC::Paper => RPC::Rock,
                    RPC::Scissors => RPC::Paper,
                }
            }
            "Y" => play2 = play1,

            "Z" => {
                play2 = match play1 {
                    RPC::Rock => RPC::Paper,
                    RPC::Paper => RPC::Scissors,
                    RPC::Scissors => RPC::Rock,
                }
            }
            _ => (),
        }
        score += get_score(&play1, &play2);
    });
    score
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(15, part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(12, part2(&input));
    }
}
