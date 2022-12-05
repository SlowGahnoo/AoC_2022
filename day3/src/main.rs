use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Rucksack {
    set1: HashSet<char>,
    set2: HashSet<char>,
}

impl Rucksack {
    fn new(sequence: &str) -> Self {
        let (first_compartment, second_compartment) = sequence.split_at(sequence.len() / 2);
        let mut set1: HashSet<char> = HashSet::new();
        let mut set2: HashSet<char> = HashSet::new();
        first_compartment.chars().for_each(|c| {
            set1.insert(c);
        });
        second_compartment.chars().for_each(|c| {
            set2.insert(c);
        });
        Rucksack { set1, set2 }
    }

    fn get_common(&self) -> Option<&char> {
        self.set1
            .intersection(&self.set2)
            .collect::<Vec<&char>>()
            .pop()
    }
    fn get_set(&self) -> HashSet<char> {
        let mut set: HashSet<char> = HashSet::new();
        set.extend(&self.set1);
        set.extend(&self.set2);
        set
    }
}

fn part1(input: &str) -> i32 {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    let mut priorities: HashMap<char, i32> = HashMap::new();

    for (priority, letter) in (b'a'..=b'z').enumerate() {
        priorities.insert(letter as char, priority as i32 + 1);
    }
    for (priority, letter) in (b'A'..=b'Z').enumerate() {
        priorities.insert(letter as char, priority as i32 + 27);
    }
    for line in input.lines() {
        rucksacks.push(Rucksack::new(line))
    }

    let mut sum = 0;
    for rucksack in rucksacks.iter() {
        sum += priorities.get(rucksack.get_common().unwrap()).unwrap();
    }
    sum
}

fn part2(input: &str) -> i32 {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    let mut priorities: HashMap<char, i32> = HashMap::new();

    for (priority, letter) in (b'a'..=b'z').enumerate() {
        priorities.insert(letter as char, priority as i32 + 1);
    }
    for (priority, letter) in (b'A'..=b'Z').enumerate() {
        priorities.insert(letter as char, priority as i32 + 27);
    }
    for line in input.lines() {
        rucksacks.push(Rucksack::new(line))
    }

    let mut sum = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        let c: char = rucksacks[i]
            .get_set()
            .intersection(&rucksacks[i + 1].get_set())
            .copied()
            .collect::<HashSet<char>>()
            .intersection(&rucksacks[i + 2].get_set())
            .copied()
            .collect::<Vec<char>>()
            .pop()
            .unwrap();
        sum += priorities.get(&c).unwrap();
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:#?}", part1(&input));
    println!("{:#?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(157, part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(70, part2(&input));
    }
}
