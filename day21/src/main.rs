use std::{collections::HashSet, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(21, input_transform)?;

    // Run parts
    println!("Part 1: {}", run(&input, 5));
    println!("Part 2: {}", run(&input, 18));

    Ok(())
}

fn run(input: &[Transform], iterations: usize) -> usize {
    let mut pic = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];

    for _ in 0..iterations {
        if pic.len() % 2 == 0 {
            pic = transform_pic(pic, 2, input);
        } else {
            pic = transform_pic(pic, 3, input);
        }
    }

    pic.iter()
        .map(|line| line.iter().filter(|p| **p).count())
        .sum()
}

fn transform_pic(in_pic: PxArray, in_chunk_size: usize, transforms: &[Transform]) -> PxArray {
    let in_size = in_pic.len();
    let chunks = in_size / in_chunk_size;
    let out_chunk_size = in_chunk_size + 1;
    let out_size = chunks * out_chunk_size;
    let mut out_pic = vec![vec![false; out_size]; out_size];

    for (ychunk, in_y) in (0..in_size).step_by(in_chunk_size).enumerate() {
        let out_y = ychunk * out_chunk_size;

        for (xchunk, in_x) in (0..in_size).step_by(in_chunk_size).enumerate() {
            let out_x = xchunk * out_chunk_size;

            let in_pixels = (0..in_chunk_size)
                .map(|yoff| in_pic[in_y + yoff][in_x..(in_x + in_chunk_size)].to_vec())
                .collect::<Vec<_>>();

            let out_pixels = match_transform(&in_pixels, in_chunk_size, transforms);

            for yoff in 0..out_chunk_size {
                for xoff in 0..out_chunk_size {
                    out_pic[out_y + yoff][out_x + xoff] = out_pixels[yoff][xoff];
                }
            }
        }
    }

    out_pic
}

fn match_transform(in_px: &PxArray, order: usize, transforms: &[Transform]) -> PxArray {
    let mut out_px = Vec::new();

    for transform in transforms.iter() {
        if transform.order != order {
            continue;
        }

        if transform.from.contains(in_px) {
            out_px = transform.to.clone();
            break;
        }
    }

    out_px
}

fn flip_h(inarr: PxArray) -> PxArray {
    inarr
        .into_iter()
        .map(|l| l.into_iter().rev().collect())
        .collect()
}

fn flip_v(inarr: PxArray) -> PxArray {
    inarr.into_iter().rev().collect()
}

fn rotate(inarr: PxArray) -> PxArray {
    let mut outarr = inarr.clone();

    for y in 0..inarr.len() {
        (0..inarr.len()).for_each(|x| outarr[x][inarr.len() - 1 - y] = inarr[y][x]);
    }

    outarr
}

type PxArray = Vec<Vec<bool>>;

#[derive(Debug)]
struct Transform {
    order: usize,
    from: HashSet<PxArray>,
    to: PxArray,
}

// Input parsing

fn parse_pattern(pattern: &str) -> PxArray {
    pattern
        .split('/')
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid char {c}"),
                })
                .collect()
        })
        .collect()
}

fn input_transform(line: String) -> Transform {
    let mut terms = line.split(" => ");

    let mut from_px = parse_pattern(terms.next().expect("From pattern not found"));
    let order = from_px.len();
    let to_px = parse_pattern(terms.next().expect("To pattern not found"));

    let mut from = HashSet::new();

    // Insert pixel permutations
    for _ in 0..2 {
        from.insert(from_px.clone());

        from_px = flip_h(from_px);
        from.insert(from_px.clone());

        from_px = flip_v(from_px);
        from.insert(from_px.clone());

        from_px = flip_h(from_px);
        from.insert(from_px.clone());

        from_px = flip_v(from_px);

        from_px = rotate(from_px);
    }

    Transform {
        order,
        from,
        to: to_px,
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(run(&input, 2), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(
            rotate(vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, true, true]
            ]),
            vec![
                vec![true, false, false],
                vec![true, false, true],
                vec![true, true, false]
            ]
        )
    }
}
