use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(13, input_transform)?;

    let firewall = FireWall::new(&input);

    // Run parts
    println!("Part 1: {}", part1(&firewall));
    println!("Part 2: {}", part2(&firewall));

    Ok(())
}

fn part1(firewall: &FireWall) -> usize {
    let mut score = 0;

    for l in 0..firewall.count {
        if let Some(0) = firewall.layer_at_time(l, l) {
            score += l * firewall.layers[l].as_ref().unwrap().range as usize
        }
    }

    score
}

fn part2(firewall: &FireWall) -> usize {
    let mut start_time = 0;

    loop {
        let mut caught = false;

        for l in 0..firewall.count {
            if let Some(0) = firewall.layer_at_time(l, start_time + l) {
                caught = true;
                break;
            }
        }

        if !caught {
            break;
        }

        start_time += 1;
    }

    start_time
}

struct FireWall {
    layers: Vec<Option<Layer>>,
    count: usize,
}

impl FireWall {
    fn new(input: &[InputEnt]) -> Self {
        let count = input.iter().map(|i| i.depth).max().expect("No layers") + 1;

        let mut layers: Vec<Option<Layer>> = (0..count).map(|_| None).collect();

        for i in input {
            layers[i.depth as usize] = Some(Layer { range: i.range })
        }

        Self {
            layers,
            count: count as usize,
        }
    }

    fn layer_at_time(&self, layer: usize, time: usize) -> Option<u16> {
        self.layers[layer]
            .as_ref()
            .map(|l| l.position_at_time(time))
    }
}

struct Layer {
    range: u16,
}

impl Layer {
    fn position_at_time(&self, time: usize) -> u16 {
        let mut pos = (time % self.period()) as u16;

        if pos >= self.range {
            pos = (self.range - 1) * 2 - pos;
        }

        pos
    }

    fn period(&self) -> usize {
        (((self.range - 2) * 2) + 2) as usize
    }
}

// Input parsing

struct InputEnt {
    depth: u16,
    range: u16,
}

fn input_transform(line: String) -> InputEnt {
    let nums = line
        .split_whitespace()
        .map(|s| {
            s.trim_end_matches(':')
                .parse::<u16>()
                .expect("Invalid number")
        })
        .collect::<Vec<u16>>();

    InputEnt {
        depth: nums[0],
        range: nums[1],
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "0: 3
1: 2
4: 4
6: 4
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let firewall = FireWall::new(&input);

        assert_eq!(part1(&firewall), 24);
        assert_eq!(part2(&firewall), 10);
    }
}
