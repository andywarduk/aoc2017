use std::{cmp::max, collections::HashMap, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let instructions = parse_input_vec(8, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));

    Ok(())
}

fn part1(instructions: &[Instruction]) -> i64 {
    let mut registers = HashMap::new();

    for i in instructions {
        let cond_reg_val = registers.get(&i.cond_reg).unwrap_or(&0);

        let cond_passed = match i.cond_op {
            CondOp::Lt => *cond_reg_val < i.cond_val,
            CondOp::Gt => *cond_reg_val > i.cond_val,
            CondOp::Le => *cond_reg_val <= i.cond_val,
            CondOp::Ge => *cond_reg_val >= i.cond_val,
            CondOp::Eq => *cond_reg_val == i.cond_val,
            CondOp::Ne => *cond_reg_val != i.cond_val,
        };

        if cond_passed {
            let reg = registers.entry(i.reg.clone()).or_insert(0);

            match i.regop {
                RegOp::Inc => *reg += i.regop_val,
                RegOp::Dec => *reg -= i.regop_val,
            }
        }
    }

    *registers.values().max().expect("No registers")
}

fn part2(instructions: &[Instruction]) -> i64 {
    let mut max_val = i64::MIN;
    let mut registers = HashMap::new();

    for i in instructions {
        let cond_reg_val = registers.get(&i.cond_reg).unwrap_or(&0);

        let cond_passed = match i.cond_op {
            CondOp::Lt => *cond_reg_val < i.cond_val,
            CondOp::Gt => *cond_reg_val > i.cond_val,
            CondOp::Le => *cond_reg_val <= i.cond_val,
            CondOp::Ge => *cond_reg_val >= i.cond_val,
            CondOp::Eq => *cond_reg_val == i.cond_val,
            CondOp::Ne => *cond_reg_val != i.cond_val,
        };

        if cond_passed {
            let reg = registers.entry(i.reg.clone()).or_insert(0);

            match i.regop {
                RegOp::Inc => *reg += i.regop_val,
                RegOp::Dec => *reg -= i.regop_val,
            }

            max_val = max(max_val, *reg);
        }
    }

    max_val
}

enum RegOp {
    Inc,
    Dec,
}

enum CondOp {
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
}

struct Instruction {
    reg: String,
    regop: RegOp,
    regop_val: i64,
    cond_reg: String,
    cond_op: CondOp,
    cond_val: i64,
}

// Input parsing

fn input_transform(line: String) -> Instruction {
    let mut iter = line.split_whitespace();

    let reg = iter.next().expect("Expecting register name").to_string();

    let regop = match iter.next().expect("Expecting register op") {
        "inc" => RegOp::Inc,
        "dec" => RegOp::Dec,
        _ => panic!("Invalid register op"),
    };

    let regop_val = iter
        .next()
        .expect("Expecting register op value")
        .parse::<i64>()
        .expect("Invalid register op value");

    // Skip "if"
    iter.next();

    let cond_reg = iter
        .next()
        .expect("Expecting condition register name")
        .to_string();

    let cond_op = match iter.next().expect("Expecting condition op") {
        "<" => CondOp::Lt,
        ">" => CondOp::Gt,
        "<=" => CondOp::Le,
        ">=" => CondOp::Ge,
        "==" => CondOp::Eq,
        "!=" => CondOp::Ne,
        _ => panic!("Invalid condition op"),
    };

    let cond_val = iter
        .next()
        .expect("Expecting condition value")
        .parse::<i64>()
        .expect("Invalid condition value");

    // TODO
    Instruction {
        reg,
        regop,
        regop_val,
        cond_reg,
        cond_op,
        cond_val,
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
";

    #[test]
    fn test1() {
        let instructions = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&instructions), 1);
        assert_eq!(part2(&instructions), 10);
    }
}
