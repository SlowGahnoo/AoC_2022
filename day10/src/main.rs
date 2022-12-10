use std::fs;
use std::str::FromStr;

enum Instruction {
    Addx,
    Noop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addx" => Ok(Instruction::Addx),
            "noop" => Ok(Instruction::Noop),
            _ => Err(()),
        }
    }
}

struct CPU {
    reg_x: i32,
    cycle: i32,
    instruction: Instruction,
    iargs: Vec<i32>,
    done: bool,
}

impl CPU {
    fn new() -> Self {
        CPU {
            reg_x: 1,
            cycle: 0,
            instruction: Instruction::Noop,
            iargs: vec![],
            done: false,
        }
    }

    fn fetch(&mut self, instruction: &str) {
        let ins: Vec<&str> = instruction.split_whitespace().collect();
        match Instruction::from_str(ins[0]) {
            Ok(Instruction::Addx) => {
                self.done = false;
                self.instruction = Instruction::Addx;
                self.iargs.push(ins[1].parse().unwrap());
            }
            Ok(Instruction::Noop) => {
                self.done = false;
                self.instruction = Instruction::Noop
            }
            _ => unreachable!(),
        }
    }

    fn execute(&mut self) {
        match self.instruction {
            Instruction::Addx => {
                if self.cycle == 2 {
                    self.reg_x += self.iargs.pop().unwrap();
                    self.done = true;
                    self.cycle = 0;
                }
                self.cycle += 1;
            }
            Instruction::Noop => self.done = true,
        }
    }
}

struct CRT {
    row: usize,
    col: usize,
    pixels: [[char; 40]; 6],
}

impl CRT {
    fn new() -> Self {
        CRT {
            row: 0,
            col: 0,
            pixels: [['.'; 40]; 6],
        }
    }

    fn draw(&mut self, sprite: &Sprite) {
        if sprite.get().contains(&(self.col as i32)) {
            self.pixels[self.row][self.col] = '#';
        }
        self.col += 1;
        if self.col == self.pixels[0].len() {
            self.col = 0;
            self.row += 1;
        }
    }
    fn to_string(&self) -> String {
        self.pixels
            .map(|row| row.iter().collect::<String>())
            .join("\n")
    }
}

struct Sprite {
    index: i32,
    size: i32,
}

impl Sprite {
    fn new() -> Self {
        Sprite { index: 0, size: 40 }
    }
    fn set(&mut self, pos: i32) {
        self.index = pos;
    }
    fn get(&self) -> [i32; 3] {
        if self.index == 0 {
            [self.index, self.index, self.index + 1]
        } else if self.index == self.size {
            [self.size - 1, self.size, self.size]
        } else {
            [self.index - 1, self.index, self.index + 1]
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut cpu = CPU::new();
    let mut cycles: i32 = 0;
    let mut signals: Vec<i32> = Vec::new();
    for line in input.lines() {
        cpu.fetch(line);
        while !cpu.done {
            cpu.execute();
            cycles += 1;
            match cycles {
                20 => signals.push(cycles * cpu.reg_x),
                60 => signals.push(cycles * cpu.reg_x),
                100 => signals.push(cycles * cpu.reg_x),
                140 => signals.push(cycles * cpu.reg_x),
                180 => signals.push(cycles * cpu.reg_x),
                220 => signals.push(cycles * cpu.reg_x),
                _ => (),
            }
        }
    }
    signals.iter().sum()
}

fn part2(input: &str) -> String {
    let mut cpu = CPU::new();
    let mut _cycles: i32 = 0;
    let mut crt = CRT::new();
    let mut sprite = Sprite::new();

    for line in input.lines() {
        cpu.fetch(line);
        while !cpu.done {
            cpu.execute();
            _cycles += 1;
            sprite.set(cpu.reg_x);
            crt.draw(&sprite);
        }
    }
    crt.to_string()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2:\n{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn cpu_test() {
        let input = vec!["noop", "addx 3", "addx -5"];
        let mut cpu = CPU::new();
        let mut cycles: i32 = 0;
        for line in input.iter() {
            cpu.fetch(line);
            while !cpu.done {
                cpu.execute();
                cycles += 1;
                match cycles {
                    1 => assert_eq!(1, cpu.reg_x),
                    2 => assert_eq!(1, cpu.reg_x),
                    3 => assert_eq!(1, cpu.reg_x),
                    4 => assert_eq!(4, cpu.reg_x),
                    5 => assert_eq!(4, cpu.reg_x),
                    6 => assert_eq!(-1, cpu.reg_x),
                    _ => (),
                }
            }
        }
    }

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        let mut cpu = CPU::new();
        let mut cycles: i32 = 0;
        let mut signals: Vec<i32> = Vec::new();
        for line in input.lines() {
            cpu.fetch(line);
            while !cpu.done {
                cpu.execute();
                cycles += 1;
                match cycles {
                    20 => {
                        assert_eq!(21, cpu.reg_x);
                        signals.push(cycles * cpu.reg_x)
                    }
                    60 => {
                        assert_eq!(19, cpu.reg_x);
                        signals.push(cycles * cpu.reg_x)
                    }
                    100 => {
                        assert_eq!(18, cpu.reg_x);
                        signals.push(cycles * cpu.reg_x)
                    }
                    140 => {
                        assert_eq!(21, cpu.reg_x);
                        signals.push(cycles * cpu.reg_x)
                    }
                    180 => {
                        assert_eq!(16, cpu.reg_x);
                        signals.push(cycles * cpu.reg_x)
                    }
                    220 => {
                        assert_eq!(18, cpu.reg_x);
                        signals.push(cycles * cpu.reg_x)
                    }
                    _ => (),
                }
            }
        }
        assert_eq!(13140, signals.iter().sum());
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        let mut cpu = CPU::new();
        let mut crt = CRT::new();
        let mut sprite = Sprite::new();
        let mut _cycles: i32 = 0;
        for line in input.lines() {
            cpu.fetch(line);
            while !cpu.done {
                cpu.execute();
                _cycles += 1;
                sprite.set(cpu.reg_x);
                crt.draw(&sprite);
            }
        }
        assert_eq!(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string(),
            crt.to_string()
        );
    }
}
