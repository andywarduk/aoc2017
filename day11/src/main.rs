use std::{cmp::max, error::Error};

use aoc::input::parse_input_line;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(11, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Dir]) -> u64 {
    let mut pos = Pos::new();

    for d in input {
        pos.move_dir(d);
    }

    pos.dist()
}

fn part2(input: &[Dir]) -> u64 {
    let mut pos = Pos::new();
    let mut max_dist = 0;

    for d in input {
        pos.move_dir(d);

        max_dist = max(max_dist, pos.dist());
    }

    max_dist
}

struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn move_dir(&mut self, dir: &Dir) {
        match dir {
            Dir::N => {
                self.x -= 1;
                self.y += 1;
            }
            Dir::NE => {
                self.y += 1;
                self.z -= 1;
            }
            Dir::SE => {
                self.z -= 1;
                self.x += 1;
            }
            Dir::S => {
                self.x += 1;
                self.y -= 1;
            }
            Dir::SW => {
                self.y -= 1;
                self.z += 1;
            }
            Dir::NW => {
                self.z += 1;
                self.x -= 1;
            }
        }
    }

    fn dist(&self) -> u64 {
        max(
            self.x.unsigned_abs(),
            max(self.y.unsigned_abs(), self.z.unsigned_abs()),
        )
    }
}

enum Dir {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

//    -x       +y
//      \  n  /
//    nw +---+ ne
//      /     \
// +z -+       +- -z
//      \     /
//    sw +---+ se
//      /  s  \
//    -y       +x

// Input parsing

fn input_transform(line: String) -> Vec<Dir> {
    line.split(',')
        .map(|dir| match dir {
            "n" => Dir::N,
            "ne" => Dir::NE,
            "se" => Dir::SE,
            "s" => Dir::S,
            "sw" => Dir::SW,
            "nw" => Dir::NW,
            _ => panic!("Unknown direction"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "ne,ne,ne";
    const EXAMPLE2: &str = "ne,ne,sw,sw";
    const EXAMPLE3: &str = "ne,ne,s,s";
    const EXAMPLE4: &str = "se,sw,se,sw,sw";

    #[test]
    fn test1() {
        assert_eq!(part1(&input_transform(EXAMPLE1.to_string())), 3);
        assert_eq!(part1(&input_transform(EXAMPLE2.to_string())), 0);
        assert_eq!(part1(&input_transform(EXAMPLE3.to_string())), 2);
        assert_eq!(part1(&input_transform(EXAMPLE4.to_string())), 3);
    }
}
