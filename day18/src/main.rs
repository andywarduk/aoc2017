use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    str::SplitWhitespace,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(18, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Instruction]) -> i64 {
    let mut registers: HashMap<char, i64> = HashMap::new();

    let mut ip = 0;
    let mut last_freq = 0;

    let get_reg = |reg, registers: &HashMap<char, i64>| *registers.get(&reg).unwrap_or(&0);

    let get_regimm = |regimm: &RegImm, registers: &HashMap<char, i64>| match regimm {
        RegImm::Imm(v) => *v,
        RegImm::Reg(reg) => get_reg(*reg, registers),
    };

    loop {
        let mut next_ip = ip + 1;

        match &input[ip] {
            Instruction::Snd(reg) => last_freq = get_reg(*reg, &registers),
            Instruction::Set(reg, regimm) => {
                let val = get_regimm(regimm, &registers);
                *registers.entry(*reg).or_insert(0) = val;
            }
            Instruction::Add(reg, regimm) => {
                let val = get_regimm(regimm, &registers);
                *registers.entry(*reg).or_insert(0) += val;
            }
            Instruction::Mul(reg, regimm) => {
                let val = get_regimm(regimm, &registers);
                *registers.entry(*reg).or_insert(0) *= val;
            }
            Instruction::Mod(reg, regimm) => {
                let val = get_regimm(regimm, &registers);
                *registers.entry(*reg).or_insert(0) %= val;
            }
            Instruction::Rcv(reg) => {
                if get_reg(*reg, &registers) != 0 {
                    break;
                }
            }
            Instruction::Jgz(regimm1, regimm2) => {
                let val = get_regimm(regimm1, &registers);

                if val > 0 {
                    let offset = get_regimm(regimm2, &registers);
                    next_ip = (ip as isize + offset as isize) as usize;
                }
            }
        }

        ip = next_ip;
    }

    last_freq
}

struct Thread {
    registers: HashMap<char, i64>,
    ip: usize,
    inqueue: VecDeque<i64>,
}

impl Thread {
    fn new(id: i64) -> Self {
        let mut registers = HashMap::new();

        registers.insert('p', id);

        Self {
            registers,
            ip: 0,
            inqueue: VecDeque::new(),
        }
    }

    fn get_reg(&self, reg: char) -> i64 {
        *self.registers.get(&reg).unwrap_or(&0)
    }

    fn get_regimm(&self, regimm: &RegImm) -> i64 {
        match regimm {
            RegImm::Imm(v) => *v,
            RegImm::Reg(reg) => self.get_reg(*reg),
        }
    }

    fn execute(&mut self, input: &[Instruction]) -> Vec<i64> {
        let mut out = Vec::new();

        loop {
            let mut next_ip = self.ip + 1;

            match &input[self.ip] {
                Instruction::Snd(reg) => out.push(self.get_reg(*reg)),
                Instruction::Set(reg, regimm) => {
                    let val = self.get_regimm(regimm);
                    *self.registers.entry(*reg).or_insert(0) = val;
                }
                Instruction::Add(reg, regimm) => {
                    let val = self.get_regimm(regimm);
                    *self.registers.entry(*reg).or_insert(0) += val;
                }
                Instruction::Mul(reg, regimm) => {
                    let val = self.get_regimm(regimm);
                    *self.registers.entry(*reg).or_insert(0) *= val;
                }
                Instruction::Mod(reg, regimm) => {
                    let val = self.get_regimm(regimm);
                    *self.registers.entry(*reg).or_insert(0) %= val;
                }
                Instruction::Rcv(reg) => {
                    if let Some(val) = self.inqueue.pop_front() {
                        *self.registers.entry(*reg).or_insert(0) = val;
                    } else {
                        break;
                    }
                }
                Instruction::Jgz(regimm1, regimm2) => {
                    let val = self.get_regimm(regimm1);

                    if val > 0 {
                        let offset = self.get_regimm(regimm2);
                        next_ip = (self.ip as isize + offset as isize) as usize;
                    }
                }
            }

            self.ip = next_ip;
        }

        out
    }
}

fn part2(input: &[Instruction]) -> usize {
    let mut t1 = Thread::new(0);
    let mut t2 = Thread::new(1);
    let mut t2vals = 0;

    loop {
        let items = t1.execute(input);
        t2.inqueue.extend(items.iter());

        let items = t2.execute(input);
        t2vals += items.len();
        t1.inqueue.extend(items.iter());

        if t1.inqueue.is_empty() {
            break;
        }
    }

    t2vals
}

enum Instruction {
    Snd(Reg),
    Set(Reg, RegImm),
    Add(Reg, RegImm),
    Mul(Reg, RegImm),
    Mod(Reg, RegImm),
    Rcv(Reg),
    Jgz(RegImm, RegImm),
}

type Reg = char;

enum RegImm {
    Reg(Reg),
    Imm(i64),
}

// Input parsing

fn input_transform(line: String) -> Instruction {
    let mut iter = line.split_whitespace();

    let parse_reg = |iter: &mut SplitWhitespace| {
        let term = iter.next().expect("Expecting register");

        assert!(term.len() == 1, "Invalid register {term}");

        term.chars().next().unwrap()
    };

    let parse_regimm = |iter: &mut SplitWhitespace| {
        let term = iter.next().expect("Expecting register");

        if term.len() == 1 && term.chars().next().unwrap().is_alphabetic() {
            RegImm::Reg(term.chars().next().unwrap())
        } else {
            RegImm::Imm(term.parse::<i64>().expect("Invalid immediate"))
        }
    };

    let opcode = iter.next().expect("Opcode not found");

    match opcode {
        "snd" => Instruction::Snd(parse_reg(&mut iter)),
        "set" => Instruction::Set(parse_reg(&mut iter), parse_regimm(&mut iter)),
        "add" => Instruction::Add(parse_reg(&mut iter), parse_regimm(&mut iter)),
        "mul" => Instruction::Mul(parse_reg(&mut iter), parse_regimm(&mut iter)),
        "mod" => Instruction::Mod(parse_reg(&mut iter), parse_regimm(&mut iter)),
        "rcv" => Instruction::Rcv(parse_reg(&mut iter)),
        "jgz" => Instruction::Jgz(parse_regimm(&mut iter), parse_regimm(&mut iter)),
        _ => panic!("Invalid opcode {opcode}"),
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
";

    const EXAMPLE2: &str = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 4);
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        assert_eq!(part2(&input), 3);
    }
}
