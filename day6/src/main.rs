use std::collections::HashMap;
use std::fs;

fn check_marker(data_buff: &str, distinct: usize) -> usize {
    let mut index: usize = 0;
    for i in 0..data_buff.chars().count() - distinct {
        let mut m: HashMap<char, i32> = HashMap::new();
        let mut found: bool = true;
        let slice = &data_buff[i..i + distinct];
        slice.chars().for_each(|c| {
            *m.entry(c).or_insert(0) += 1;
        });

        m.values().for_each(|v| {
            if *v > 1 {
                found = false;
            }
        });

        if found {
            index = i + distinct;
            break;
        }
    }
    index
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", check_marker(&input, 4));
    println!("Part 2: {}", check_marker(&input, 14));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_test() {
        let data_buffers: Vec<&str> = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let markers: Vec<usize> = vec![7, 5, 6, 10, 11];
        for (b, m) in data_buffers.iter().zip(markers.iter()) {
            assert_eq!(*m, check_marker(b, 4));
        }
    }

    #[test]
    fn part2_test() {
        let data_buffers: Vec<&str> = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let markers: Vec<usize> = vec![19, 23, 23, 29, 26];
        for (b, m) in data_buffers.iter().zip(markers.iter()) {
            assert_eq!(*m, check_marker(b, 14));
        }
    }
}
