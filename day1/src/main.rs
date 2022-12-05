use std::fs;

fn part1(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|item| item.iter().sum()).max().unwrap()
}

fn part2(input: &Vec<Vec<i32>>) -> i32 {
    let mut u: Vec<i32> = input.iter().map(|item| item.iter().sum::<i32>()).collect();
    u.sort();
    u.iter().rev().take(3).sum()
}

fn parse_input(filename: &str) -> Vec<Vec<i32>> {
    let file: String = fs::read_to_string(filename).unwrap();
    file.split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|n| n.parse::<i32>().unwrap_or_default())
                .collect()
        })
        .collect()
}

fn main() {
    let input = parse_input("input.txt");
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_test() {
        let items = parse_input("test.txt");
        assert_eq!(24000, part1(&items));
    }

    #[test]
    fn part2_test() {
        let items = parse_input("test.txt");
        assert_eq!(45000, part2(&items));
    }
}
