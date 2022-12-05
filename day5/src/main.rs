use std::collections::VecDeque;
use std::fs;

struct Instruction {
    items: usize,
    from: usize,
    to: usize,
}

type Crate = VecDeque<char>;

impl Instruction {
    fn new(instruction: &str) -> Self {
        let mut tokens = instruction.split_whitespace();
        let items: usize = tokens
            .nth(1)
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let from: usize = tokens
            .nth(1)
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let to: usize = tokens
            .nth(1)
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        Instruction { items, from, to }
    }
}

fn initialize_crates(crate_order: &str) -> Vec<Crate> {
    let mut crates: Vec<Crate> = Vec::new();
    let size = crate_order
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();
    for _ in 0..size {
        crates.push(Crate::new());
    }
    let ss = crate_order.split("\n").collect::<Vec<&str>>();

    for line in ss[0..ss.len() - 1].iter().rev() {
        let ll = line.replace("    ", " [-] ");
        for (i, token) in ll.split_whitespace().enumerate() {
            if token != "[-]" {
                crates[i].push_back(token.chars().nth(1).unwrap_or_default());
            }
        }
    }
    crates
}

fn initialize_instructions(instructions: &str) -> Vec<Instruction> {
    let mut ret: Vec<Instruction> = Vec::new();
    instructions
        .split('\n')
        .for_each(|instruction| ret.push(Instruction::new(instruction)));
    ret
}

fn read_top(crates: &Vec<Crate>) -> String {
    let mut output: Vec<char> = Vec::new();
    crates.iter().for_each(|s| output.push(*s.back().unwrap()));
    output.iter().collect::<String>()
}

fn part1(input: &str) -> String {
    let [st, ins]: [&str; 2] = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();

    let mut crates: Vec<Crate> = initialize_crates(&st);
    let instructions: Vec<Instruction> = initialize_instructions(&ins);
    for Instruction { items, from, to } in instructions.iter() {
        let mut tmp_stack: Crate = Crate::new();
        for _ in 0..*items {
            tmp_stack.push_back(crates[from - 1].pop_back().unwrap());
            crates[to - 1].push_back(tmp_stack.pop_back().unwrap());
        }
    }

    read_top(&crates)
}

fn part2(input: &str) -> String {
    let [st, ins]: [&str; 2] = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();

    let mut crates: Vec<Crate> = initialize_crates(&st);
    let instructions: Vec<Instruction> = initialize_instructions(&ins);
    for Instruction { items, from, to } in instructions.iter() {
        let mut tmp_stack: Crate = Crate::new();
        for _ in 0..*items {
            tmp_stack.push_front(crates[from - 1].pop_back().unwrap());
        }
        for _ in 0..*items {
            crates[to - 1].push_back(tmp_stack.pop_front().unwrap());
        }
    }

    read_top(&crates)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!("CMZ", part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!("MCD", part2(&input));
    }
}
