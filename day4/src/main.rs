use std::{fs, ops::RangeInclusive};

#[derive(Debug)]
struct AssignmentPairs {
    first: RangeInclusive<i32>,
    second: RangeInclusive<i32>,
}

impl AssignmentPairs {
    fn new(first: Vec<i32>, second: Vec<i32>) -> Self {
        AssignmentPairs {
            first: (first[0]..=first[1]),
            second: (second[0]..=second[1]),
        }
    }

    fn check_full_overlap(&self) -> bool {
        (self.first.contains(&self.second.start()) && self.first.contains(&self.second.end()))
            || (self.second.contains(&self.first.start())
                && self.second.contains(&self.first.end()))
    }
    fn check_partial_overlap(&self) -> bool {
        self.first.contains(&self.second.start()) || self.second.contains(&self.first.start())
    }
}

fn initialize_pairs(input: &str) -> Vec<AssignmentPairs>{
    let mut assignment_pairs: Vec<AssignmentPairs> = Vec::new();
    for line in input.lines() {
        let [first, second]: [Vec<i32>; 2] = line
            .split(",")
            .map(|pair| {
                pair.split("-")
                    .map(|n| n.parse::<i32>().unwrap_or_default())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>()
            .try_into()
            .unwrap();
        assignment_pairs.push(AssignmentPairs::new(first, second));
    }
    assignment_pairs
}

fn part1(assignment_pairs: &Vec<AssignmentPairs>) -> i32 {
    assignment_pairs
        .iter()
        .fold(0, |sum, pair| sum + pair.check_full_overlap() as i32)
}

fn part2(assignment_pairs: &Vec<AssignmentPairs>) -> i32 {
    assignment_pairs
        .iter()
        .fold(0, |sum, pair| sum + pair.check_partial_overlap() as i32)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let assignment_pairs: Vec<AssignmentPairs> = initialize_pairs(&input);
    println!("Part 1: {}", part1(&assignment_pairs));
    println!("Part 2: {}", part2(&assignment_pairs));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        let assignment_pairs = initialize_pairs(&input);
        assert_eq!(2, part1(&assignment_pairs));
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        let assignment_pairs = initialize_pairs(&input);
        assert_eq!(4, part2(&assignment_pairs));
    }
}
