use std::collections::{HashSet, VecDeque};
use std::fs;

fn is_valid(grid: &Vec<Vec<char>>, point: (i32, i32)) -> bool {
    let (x, y) = point;
    (0..grid[0].len()).contains(&(x as usize)) && (0..grid.len()).contains(&(y as usize))
}

fn find_path(grid: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> Option<i32> {
    let (x, y) = start;
    let steps = 0;
    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    q.push_back((steps, x, y));
    visited.insert((x, y));

    while !q.is_empty() {
        let (steps, x, y) = q.pop_front().unwrap();

        if (x, y) == end {
            return Some(steps);
        }

        let neighbours = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for n in neighbours.iter() {
            let (nx, ny) = n;
            if is_valid(grid, *n) && !visited.contains(&(*nx, *ny)) {
                if grid[*ny as usize][*nx as usize] as i32
                    <= grid[y as usize][x as usize] as i32 + 1
                {
                    q.push_back((steps + 1, *nx, *ny));
                    visited.insert((*nx, *ny));
                }
            }
        }
    }
    None
}

fn part1(input: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> i32 {
    find_path(&input, start, end).unwrap()
}

fn part2(input: &Vec<Vec<char>>, end: (i32, i32)) -> i32 {
    let mut starts: Vec<(i32, i32)> = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            match input[y][x] {
                'a' => {
                    starts.push((x as i32, y as i32));
                }
                _ => (),
            }
        }
    }
    starts
        .iter()
        .map(|s| match find_path(&input, *s, end) {
            Some(steps) => steps,
            None => std::i32::MAX,
        })
        .min()
        .unwrap()
}

fn parse_input(filename: &str) -> (Vec<Vec<char>>, (i32, i32), (i32, i32)) {
    let mut input: Vec<Vec<char>> = fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|l| l.chars().collect())
        .collect();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            match input[y][x] {
                'S' => {
                    input[y][x] = 'a';
                    start = (x as i32, y as i32);
                }
                'E' => {
                    input[y][x] = 'z';
                    end = (x as i32, y as i32);
                }
                _ => (),
            }
        }
    }
    (input, start, end)
}

fn main() {
    let (input, start, end) = parse_input("input.txt");
    println!("Part 1 {}", part1(&input, start, end));
    println!("Part 2 {}", part2(&input, end));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_test() {
        let (input, start, end) = parse_input("test.txt");
        assert_eq!(31, part1(&input, start, end));
    }
    #[test]
    fn part2_test() {
        let (input, _start, end) = parse_input("test.txt");
        assert_eq!(29, part2(&input, end));
    }
}
