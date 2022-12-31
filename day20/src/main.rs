use std::{collections::HashMap, error::Error, ops::AddAssign};

use aoc::input::parse_input_vec;

use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(20, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Particle]) -> usize {
    let (nearest, _) =
        input
            .iter()
            .enumerate()
            .fold((0, f64::MAX), |(nearest, min_accel), (i, p)| {
                let accel = (((p.accel.x * p.accel.x)
                    + (p.accel.y * p.accel.y)
                    + (p.accel.z * p.accel.z)) as f64)
                    .sqrt();
                if accel < min_accel {
                    (i, accel)
                } else {
                    (nearest, min_accel)
                }
            });

    nearest
}

fn part2(input: &[Particle]) -> usize {
    let mut particles = input.to_vec();

    loop {
        let mut positions = HashMap::new();

        (0..particles.len()).for_each(|i| {
            let accel = particles[i].accel;
            particles[i].velocity += accel;
            let velocity = particles[i].velocity;
            particles[i].position += velocity;

            positions
                .entry(particles[i].position)
                .or_insert_with(Vec::new)
                .push(i);
        });

        let mut rmlist = positions
            .into_iter()
            .flat_map(|(_, elems)| if elems.len() > 1 { elems } else { vec![] })
            .collect::<Vec<_>>();

        rmlist.sort();

        for rm in rmlist.into_iter().rev() {
            particles.swap_remove(rm);
        }

        if particles.iter().all(|p| !p.changing_direction()) {
            break;
        }
    }

    particles.len()
}

#[derive(Debug, Clone)]
struct Particle {
    position: Triple,
    velocity: Triple,
    accel: Triple,
}

impl Particle {
    fn changing_direction(&self) -> bool {
        (self.accel.x != 0 && self.velocity.x.signum() != self.accel.x.signum())
            || (self.accel.y != 0 && self.velocity.y.signum() != self.accel.y.signum())
            || (self.accel.z != 0 && self.velocity.z.signum() != self.accel.z.signum())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Triple {
    x: i64,
    y: i64,
    z: i64,
}

impl AddAssign for Triple {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// Input parsing

fn input_transform(line: String) -> Particle {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^p=<(-?\d*),(-?\d*),(-?\d*)>, v=<(-?\d*),(-?\d*),(-?\d*)>, a=<(-?\d*),(-?\d*),(-?\d*)>$"
        )
        .unwrap();
    }

    let nums: Vec<i64> = RE
        .captures(&line)
        .unwrap_or_else(|| panic!("Invalid input line: {line}"))
        .iter()
        .skip(1)
        .map(|m| {
            m.expect("No match")
                .as_str()
                .parse::<i64>()
                .expect("Invalid number")
        })
        .collect();

    Particle {
        position: Triple {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        },
        velocity: Triple {
            x: nums[3],
            y: nums[4],
            z: nums[5],
        },
        accel: Triple {
            x: nums[6],
            y: nums[7],
            z: nums[8],
        },
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "TODO";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 0 /* TODO */);
        assert_eq!(part2(&input), 0 /* TODO */);
    }
}
