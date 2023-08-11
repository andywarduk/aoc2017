use std::{collections::HashMap, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(7, input_transform)?;

    let programs = Programs::new(&input);

    // Run parts
    println!("Part 1: {}", part1(&programs));
    println!("Part 2: {}", part2(&programs));

    Ok(())
}

fn part1(programs: &Programs) -> &String {
    &programs.root
}

fn part2(programs: &Programs) -> u64 {
    programs.find_unbalanced().unwrap()
}

struct Programs {
    programs: HashMap<String, Program>,
    root: String,
}

impl Programs {
    fn new(input: &[Program]) -> Self {
        // Create hash map
        let mut programs: HashMap<String, Program> =
            input.iter().map(|p| (p.name.clone(), p.clone())).collect();

        // Calculate parentage
        let parents = programs
            .values()
            .flat_map(|p| p.children.iter().map(|c| (p.name.clone(), c.clone())))
            .collect::<Vec<_>>();

        // Apply parents
        for (parent, child) in parents {
            programs
                .get_mut(&child)
                .expect("Child node not found")
                .parent = parent;
        }

        // Find root node
        let root = programs
            .values()
            .find_map(|p| {
                if p.parent.is_empty() {
                    Some(p.name.clone())
                } else {
                    None
                }
            })
            .expect("Root not found");

        // Calculate total weights
        let mut tot_weights = HashMap::new();

        for p in programs.values() {
            Self::calc_tot_weight(&programs, &p.name, &mut tot_weights);
        }

        // Apply total weights
        for (n, v) in tot_weights {
            programs.get_mut(&n).unwrap().tot_weight = v;
        }

        Programs { programs, root }
    }

    fn calc_tot_weight(
        programs: &HashMap<String, Program>,
        name: &str,
        tot_weights: &mut HashMap<String, u64>,
    ) -> u64 {
        match tot_weights.get(name) {
            None => {
                let tw = programs[name].weight
                    + programs[name]
                        .children
                        .iter()
                        .map(|c| Self::calc_tot_weight(programs, c, tot_weights))
                        .sum::<u64>();

                tot_weights.insert(name.to_string(), tw);

                tw
            }
            Some(v) => *v,
        }
    }

    fn find_unbalanced(&self) -> Option<u64> {
        self.find_unbalanced_iter(&self.root)
    }

    fn find_unbalanced_iter(&self, node: &str) -> Option<u64> {
        let node = &self.programs[node];

        // Build set of weight counts
        let mut weight_set = HashMap::new();

        for c in node.children.iter() {
            let weight = &self.programs[c].tot_weight;
            *weight_set.entry(*weight).or_insert(0) += 1usize;
        }

        match weight_set.len() {
            0 => None,
            1 | 2 => {
                // Iterate children
                for c in node.children.iter() {
                    if let Some(v) = self.find_unbalanced_iter(c) {
                        return Some(v);
                    }
                }

                if weight_set.len() == 2 {
                    // This node is incorrect
                    let incorrect_weight = weight_set
                        .iter()
                        .find_map(|(weight, count)| if *count == 1 { Some(*weight) } else { None })
                        .expect("Incorrect weight not found");

                    let correct_weight = weight_set
                        .iter()
                        .find_map(|(weight, count)| if *count != 1 { Some(*weight) } else { None })
                        .expect("Correct weight not found");

                    let weight_adj = correct_weight as i64 - incorrect_weight as i64;

                    // Which child has the incorrect weight?
                    let incorrect_child = node
                        .children
                        .iter()
                        .find_map(|c| {
                            let program = &self.programs[c];

                            if program.tot_weight == incorrect_weight {
                                Some(program)
                            } else {
                                None
                            }
                        })
                        .expect("Incorrect program not found");

                    // Return corrected weight
                    Some((incorrect_child.weight as i64 + weight_adj) as u64)
                } else {
                    None
                }
            }
            _ => panic!("Can't determine incorrect weight"),
        }
    }
}

// Input parsing

#[derive(Debug, Clone)]
struct Program {
    name: String,
    weight: u64,
    tot_weight: u64,
    children: Vec<String>,
    parent: String,
}

fn input_transform(line: String) -> Program {
    let mut iter = line.split_whitespace();

    let name = iter.next().expect("No program name").to_string();

    let weight = iter
        .next()
        .expect("No weight")
        .trim_start_matches('(')
        .trim_end_matches(')')
        .parse::<u64>()
        .expect("Invalid weight");

    let children = match iter.next() {
        None => vec![],
        Some("->") => iter.map(|c| c.trim_end_matches(',').to_string()).collect(),
        _ => panic!("Invalid children spec"),
    };

    Program {
        name,
        weight,
        tot_weight: 0,
        children,
        parent: "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        let programs = Programs::new(&input);

        assert_eq!(part1(&programs), "tknk");
        assert_eq!(part2(&programs), 60);
    }
}
