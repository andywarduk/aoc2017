use std::{collections::HashMap, slice::Iter};

use lazy_static::lazy_static;
use regex::Regex;

use crate::{Action, Dir, Program, State};

pub fn parse_program(input: &[String]) -> Program {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"^Begin in state (.).$").unwrap();
        static ref RE2: Regex =
            Regex::new(r"^Perform a diagnostic checksum after (\d*) steps.$").unwrap();
        static ref RE3: Regex = Regex::new(r"^In state (.):$").unwrap();
    }

    let mut iter = input.iter();

    // Parse first line
    let init_state = get_char(&RE1, &mut iter);

    // Parse second line
    let diag_steps = get_int(&RE2, &mut iter);

    let mut states = HashMap::new();

    while let Some(line) = iter.next() {
        assert!(line.is_empty());

        let id = get_char(&RE3, &mut iter);

        let zero = parse_action(&mut iter, false);
        let one = parse_action(&mut iter, true);

        states.insert(id, State { zero, one });
    }

    Program {
        init_state,
        diag_steps,
        states,
    }
}

fn parse_action(iter: &mut Iter<String>, value: bool) -> Action {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"^    - Write the value (\d).$").unwrap();
        static ref RE2: Regex = Regex::new(r"^    - Move one slot to the ([^\.]*).$").unwrap();
        static ref RE3: Regex = Regex::new(r"^    - Continue with state (.).$").unwrap();
    }

    let line = iter.next().expect("Line missing");

    let expected = format!("  If the current value is {}:", u8::from(value));
    assert_eq!(line, &expected);

    let val = get_int(&RE1, iter);
    let write = val == 1;

    let dir_str = get_string(&RE2, iter);

    let mv = match dir_str {
        "right" => Dir::Right,
        "left" => Dir::Left,
        _ => panic!("Invalid direction"),
    };

    let next = get_char(&RE3, iter);

    Action { write, mv, next }
}

fn get_char(re: &Regex, iter: &mut Iter<String>) -> char {
    let line = iter.next().expect("Lines exhausted");

    get_char_from_str(re, line)
}

fn get_char_from_str(re: &Regex, line: &str) -> char {
    get_string_from_str(re, line)
        .chars()
        .next()
        .expect("No characters")
}

fn get_int(re: &Regex, iter: &mut Iter<String>) -> u64 {
    let line = iter.next().expect("Lines exhausted");

    get_int_from_str(re, line)
}

fn get_int_from_str(re: &Regex, line: &str) -> u64 {
    get_string_from_str(re, line)
        .parse::<u64>()
        .expect("Invalid number")
}

fn get_string<'a>(re: &Regex, iter: &'a mut Iter<String>) -> &'a str {
    let line = iter.next().expect("Lines exhausted");

    get_string_from_str(re, line)
}

fn get_string_from_str<'a>(re: &Regex, line: &'a str) -> &'a str {
    re.captures(line)
        .unwrap_or_else(|| panic!("Invalid input line: {line}"))
        .iter()
        .nth(1)
        .expect("No match")
        .map(|m| m.as_str())
        .expect("Map failed")
}

pub fn input_transform(line: String) -> String {
    line
}
