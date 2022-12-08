use std::fs;

fn is_visible(x: usize, y: usize, tree_map: &Vec<Vec<u32>>) -> bool {
    let h = tree_map[y][x];
    let mut left: bool = false;
    let mut right: bool = false;
    let mut top: bool = false;
    let mut bottom: bool = false;

    for i in 0..x {
        if tree_map[y][i] >= h {
            left = false;
            break;
        } else {
            left = true
        };
    }
    for i in x + 1..tree_map[0].len() {
        if tree_map[y][i] >= h {
            right = false;
            break;
        } else {
            right = true
        };
    }
    for i in 0..y {
        if tree_map[i][x] >= h {
            top = false;
            break;
        } else {
            top = true
        };
    }
    for i in y + 1..tree_map.len() {
        if tree_map[i][x] >= h {
            bottom = false;
            break;
        } else {
            bottom = true
        };
    }
    top | bottom | left | right
}

fn get_score(x: usize, y: usize, tree_map: &Vec<Vec<u32>>) -> usize {
    let h = tree_map[y][x];
    let mut sxr: usize = 0;
    let mut sxl: usize = 0;
    let mut syu: usize = 0;
    let mut syd: usize = 0;

    for i in (0..x).rev() {
        sxl += 1;
        if tree_map[y][i] >= h {
            break;
        }
    }
    for i in x + 1..tree_map[0].len() {
        sxr += 1;
        if tree_map[y][i] >= h {
            break;
        }
    }
    for i in (0..y).rev() {
        syu += 1;
        if tree_map[i][x] >= h {
            break;
        }
    }
    for i in y + 1..tree_map.len() {
        syd += 1;
        if tree_map[i][x] >= h {
            break;
        }
    }
    sxr * sxl * syd * syu
}

fn part1(input: &str) -> usize {
    let tree_map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut total = (tree_map.len() - 1) * 2 + (tree_map[0].len() - 1) * 2;
    for y in 1..tree_map.len() - 1 {
        for x in 1..tree_map[0].len() - 1 {
            if is_visible(x, y, &tree_map) {
                total += 1;
            }
        }
    }
    total
}

fn part2(input: &str) -> usize {
    let tree_map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut scores: Vec<usize> = vec![];
    for y in 1..tree_map.len() - 1 {
        for x in 1..tree_map[0].len() - 1 {
            scores.push(get_score(x, y, &tree_map));
        }
    }
    *scores.iter().max().unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(21, part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(8, part2(&input));
    }
}
