use std::{error::Error, str::SplitWhitespace};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(23, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2());

    Ok(())
}

fn part1(input: &[Instruction]) -> u64 {
    let mut registers: [i64; 8] = [0; 8];

    let mut ip = 0;

    let mut mulins = 0;

    let get_reg = |reg: char, registers: &[i64]| registers[(reg as u8 - b'a') as usize];

    let set_reg =
        |reg: char, val: i64, registers: &mut [i64]| registers[(reg as u8 - b'a') as usize] = val;

    let get_regimm = |regimm: &RegImm, registers: &[i64]| match regimm {
        RegImm::Imm(v) => *v,
        RegImm::Reg(reg) => get_reg(*reg, registers),
    };

    while ip < input.len() {
        let mut next_ip = ip + 1;

        match &input[ip] {
            Instruction::Set(reg, regimm) => {
                let val = get_regimm(regimm, &registers);
                set_reg(*reg, val, &mut registers);
            }
            Instruction::Sub(reg, regimm) => {
                let val = get_regimm(regimm, &registers);
                set_reg(*reg, get_reg(*reg, &registers) - val, &mut registers);
            }
            Instruction::Mul(reg, regimm) => {
                let val = get_regimm(regimm, &registers);
                set_reg(*reg, get_reg(*reg, &registers) * val, &mut registers);
                mulins += 1;
            }
            Instruction::Jnz(regimm1, regimm2) => {
                let val = get_regimm(regimm1, &registers);

                if val != 0 {
                    let offset = get_regimm(regimm2, &registers);
                    next_ip = (ip as isize + offset as isize) as usize;
                }
            }
        }

        ip = next_ip;
    }

    mulins
}

fn part2() -> i32 {
    let mut b = (93 * 100) + 100_000;
    let c = b + 17_000;
    let mut d;
    let mut f;
    let mut h = 0;

    loop {
        f = 1;
        d = 2;

        while d < b {
            if b % d == 0 {
                f = 0;
                break;
            }

            d += 1;
        }

        if f == 0 {
            h += 1;
        }

        if b - c == 0 {
            break;
        }

        b += 17;
    }

    h
}

enum Instruction {
    Set(Reg, RegImm),
    Sub(Reg, RegImm),
    Mul(Reg, RegImm),
    Jnz(RegImm, RegImm),
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
        "set" => Instruction::Set(parse_reg(&mut iter), parse_regimm(&mut iter)),
        "sub" => Instruction::Sub(parse_reg(&mut iter), parse_regimm(&mut iter)),
        "mul" => Instruction::Mul(parse_reg(&mut iter), parse_regimm(&mut iter)),
        "jnz" => Instruction::Jnz(parse_regimm(&mut iter), parse_regimm(&mut iter)),
        _ => panic!("Invalid opcode {opcode}"),
    }
}
