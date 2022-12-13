use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::str::FromStr;
use std::{fmt, fs, vec};

#[derive(Debug)]
struct Tree {
    root: Rc<RefCell<Node>>,
}
impl Tree {
    fn new() -> Self {
        Tree {
            root: Rc::new(RefCell::new(Node {
                level: 0,
                line: "".to_string(),
                children: vec![],
            })),
        }
    }
}

#[derive(Debug)]
struct Node {
    level: usize,
    line: String,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(level: usize, line: &str) -> Self {
        Node {
            level,
            line: line.to_string(),
            children: vec![],
        }
    }
    fn add_child(&mut self, line: &str) {
        let new_node = Rc::new(RefCell::new(Node::new(self.level + 1, line)));
        self.children.push(new_node);
    }
}

fn parse_to_tree(input: &str, tree: &Tree) {
    let s = input.split("\n\n").collect::<VecDeque<&str>>();
    let mut _tree_ptr = Rc::clone(&tree.root);
    let mut prev_ident_lvl = 0;
    for structure in s.iter() {
        _tree_ptr = Rc::clone(&tree.root);
        for line in structure.lines() {
            let ident_lvl = line.matches("  ").count();
            if ident_lvl > prev_ident_lvl {
                let prev_ptr = Rc::clone(&_tree_ptr);
                _tree_ptr = Rc::clone(&prev_ptr.borrow().children.last().unwrap());
                _tree_ptr.borrow_mut().add_child(line);
            } else {
                _tree_ptr.borrow_mut().add_child(line);
            }
            prev_ident_lvl = ident_lvl;
        }
        prev_ident_lvl = 0;
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monkey {}:\n  items: {:?}", self.id, self.items)
    }
}

#[derive(Debug)]
enum Operation {
    Add((Value, Value)),
    Mul((Value, Value)),
}

#[derive(Debug)]
enum Value {
    Old,
    Num(usize),
}

#[derive(Debug)]
struct Test {
    div: usize,
    id_true: usize,
    id_false: usize,
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    worry: usize,
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
    inspections: usize,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let expression = s.split("=").last().unwrap();
        let tokens = expression.split_whitespace().collect::<Vec<&str>>();
        match tokens[1] {
            "*" => {
                let op1 = tokens[0];
                let op2 = tokens[2];
                let oop1 = match op1 {
                    "old" => Value::Old,
                    num => Value::Num(num.parse().unwrap()),
                };
                let oop2 = match op2 {
                    "old" => Value::Old,
                    num => Value::Num(num.parse().unwrap()),
                };
                Ok(Operation::Mul((oop1, oop2)))
            }
            "+" => {
                let op1 = tokens[0];
                let op2 = tokens[2];
                let oop1 = match op1 {
                    "old" => Value::Old,
                    num => Value::Num(num.parse().unwrap()),
                };
                let oop2 = match op2 {
                    "old" => Value::Old,
                    num => Value::Num(num.parse().unwrap()),
                };
                Ok(Operation::Add((oop1, oop2)))
            }
            _ => Err(()),
        }
    }
}

impl Monkey {
    fn parse_test(test_node: &Node) -> Test {
        let div = test_node
            .line
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let monkey_id = &test_node
            .children
            .iter()
            .map(|n| {
                n.borrow()
                    .line
                    .to_string()
                    .pop()
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize
            })
            .collect::<Vec<usize>>();
        Test {
            div,
            id_true: monkey_id[0] as usize,
            id_false: monkey_id[1] as usize,
        }
    }

    fn new(node: &Node) -> Self {
        let id = node
            .line
            .split_whitespace()
            .last()
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let items: VecDeque<usize> = node.children[0]
            .borrow()
            .line
            .split(":")
            .last()
            .unwrap()
            .split(",")
            .map(|n| n.strip_prefix(" ").unwrap().parse::<usize>().unwrap())
            .collect();

        let operation = Operation::from_str(&node.children[1].borrow().line).unwrap();
        let test = Self::parse_test(&node.children[2].borrow());

        Monkey {
            id,
            worry: 0,
            items,
            operation,
            test,
            inspections: 0,
        }
    }
    fn inspect(&mut self, lcm: usize, part1: bool) -> usize {
        self.inspections += 1;
        self.worry = self.items.pop_front().unwrap();
        let result = match &self.operation {
            Operation::Mul((a, b)) => {
                let op1 = match a {
                    Value::Old => self.worry,
                    Value::Num(num) => *num,
                };
                let op2 = match b {
                    Value::Old => self.worry,
                    Value::Num(num) => *num,
                };
                op1 * op2
            }
            Operation::Add((a, b)) => {
                let op1 = match a {
                    Value::Old => self.worry,
                    Value::Num(num) => *num,
                };
                let op2 = match b {
                    Value::Old => self.worry,
                    Value::Num(num) => *num,
                };
                op1 + op2
            }
        };
        self.worry = result % lcm;
        if part1 {
            self.worry = self.worry / 3;
        }
        self.worry
    }

    fn test(&self) -> (usize, usize) {
        if self.worry % self.test.div == 0 {
            (self.worry, self.test.id_true)
        } else {
            (self.worry, self.test.id_false)
        }
    }
}

fn part1(input: &str) -> usize {
    let tree = Tree::new();
    parse_to_tree(&input, &tree);
    let mut monkeys: Vec<Monkey> = vec![];
    for m in tree.root.borrow().children.iter() {
        monkeys.push(Monkey::new(&m.borrow()));
    }
    let lcm = monkeys
        .iter()
        .map(|m| m.test.div)
        .fold(1, |mut _llcm, value| {
            _llcm *= value;
            _llcm
        });

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() != 0 {
                monkeys[i].inspect(lcm, true);
                let (w, id) = monkeys[i].test();
                monkeys[id].items.push_back(w);
            }
        }
    }

    let mut activity = monkeys
        .iter()
        .map(|m| m.inspections)
        .collect::<Vec<usize>>();
    activity.sort();
    activity.reverse();
    activity[0] * activity[1]
}

fn part2(input: &str) -> usize {
    let tree = Tree::new();
    parse_to_tree(&input, &tree);
    let mut monkeys: Vec<Monkey> = vec![];
    for m in tree.root.borrow().children.iter() {
        monkeys.push(Monkey::new(&m.borrow()));
    }
    let lcm = monkeys
        .iter()
        .map(|m| m.test.div)
        .fold(1, |mut _llcm, value| {
            _llcm *= value;
            _llcm
        });

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() != 0 {
                monkeys[i].inspect(lcm, false);
                let (w, id) = monkeys[i].test();
                monkeys[id].items.push_back(w);
            }
        }
    }

    let mut activity = monkeys
        .iter()
        .map(|m| m.inspections)
        .collect::<Vec<usize>>();
    activity.sort();
    activity.reverse();
    activity[0] * activity[1]

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
        assert_eq!(10605, part1(&input));
    }
    #[test]
    fn part2_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(2713310158, part2(&input));
    }
}
