use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Snek {
    tails: Vec<Tail>,
}

impl Snek {
    fn new() -> Self {
        let tails = vec![Tail::new(0, 0)];
        Snek { tails }
    }

    fn add_tail(&mut self) {
        self.tails
            .push(Tail::new(self.tails.len(), self.tails.len() - 1));
    }

    fn goto(&mut self, command: &(char, i32)) {
        let (direction, steps) = command;
        for _ in 0..*steps {
            match direction {
                'U' => self.tails[0].head.1 += 1,
                'D' => self.tails[0].head.1 -= 1,
                'L' => self.tails[0].head.0 -= 1,
                'R' => self.tails[0].head.0 += 1,
                _ => unreachable!(),
            }

            for i in 0..self.tails.len() {
                let parent = self.tails[i].parent;
                if self.tails[i].id != 0 {
                    let prev = self.tails[parent].head;
                    self.tails[i].drag_tail(prev);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Tail {
    id: usize,
    head: (i32, i32),
    parent: usize,
    visited: HashSet<(i32, i32)>,
}

impl Tail {
    fn new(id: usize, parent: usize) -> Self {
        Tail {
            id,
            head: (0, 0),
            parent,
            visited: HashSet::new(),
        }
    }

    fn distance(&self, prev: (i32, i32)) -> (i32, i32) {
        let tail = &self.head;
        let dx = prev.0 - tail.0;
        let dy = prev.1 - tail.1;
        (dx, dy)
    }

    fn is_touching(&self, dx: i32, dy: i32) -> bool {
        let border = [
            (-1, 1),
            (0, 1),
            (1, 1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        border.contains(&(dx, dy))
    }

    fn drag_tail(&mut self, prev: (i32, i32)) {
        let (dx, dy) = self.distance(prev);
        if !self.is_touching(dx, dy) {
            let tail = &mut self.head;
            *tail = (
                tail.0 + (prev.0 != tail.0) as i32 * dx.signum(),
                tail.1 + (prev.1 != tail.1) as i32 * dy.signum(),
            );
        }
        self.visited.insert(self.head);
    }

    fn get_visited(&self) -> usize {
        self.visited.len()
    }
}

fn part1(input: &str) -> usize {
    let mut commands: Vec<(char, i32)> = vec![];
    input.lines().for_each(|l| {
        let ll = l.split_whitespace().collect::<Vec<&str>>();
        let c = ll[0].chars().last().unwrap();
        let n = ll[1].parse::<i32>().unwrap();
        commands.push((c, n));
    });
    let mut snek = Snek::new();
    snek.add_tail();
    commands.iter().for_each(|command| {
        snek.goto(&command);
    });
    let tail = snek.tails.last().unwrap();
    tail.get_visited()
}

fn part2(input: &str) -> usize {
    let mut commands: Vec<(char, i32)> = vec![];
    input.lines().for_each(|l| {
        let ll = l.split_whitespace().collect::<Vec<&str>>();
        let c = ll[0].chars().last().unwrap();
        let n = ll[1].parse::<i32>().unwrap();
        commands.push((c, n));
    });
    let mut snek = Snek::new();
    (0..9).for_each(|_| snek.add_tail());
    commands.iter().for_each(|command| {
        snek.goto(&command);
    });
    let tail = snek.tails.last().unwrap();
    tail.get_visited()
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
        assert_eq!(13, part1(&input));
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("test2.txt").unwrap();
        assert_eq!(36, part2(&input));
    }
}
