// Knot hasher

use std::collections::VecDeque;

use itertools::Itertools;

pub fn knot_hash(input: &str) -> Vec<u8> {
    let twists = input
        .as_bytes()
        .iter()
        .map(|b| *b as usize)
        .chain([17, 31, 73, 47, 23])
        .collect::<Vec<_>>();

    let mut queue = (0..=255).collect::<VecDeque<u8>>();
    let mut rotated = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        twist(&twists, &mut queue, &mut skip_size, &mut rotated);
    }

    queue.rotate_right(rotated % queue.len());

    let hash = queue
        .iter()
        .chunks(16)
        .into_iter()
        .map(|chunk| chunk.fold(0, |acc, byte| acc ^ *byte))
        .collect::<Vec<_>>();

    hash
}

fn twist(twists: &[usize], queue: &mut VecDeque<u8>, skip_size: &mut usize, rotated: &mut usize) {
    for twist in twists.iter() {
        let mut new_queue = queue.split_off(*twist);
        new_queue.extend(queue.iter().rev());
        *queue = new_queue;
        queue.rotate_left(*skip_size % queue.len());
        *rotated += *twist + *skip_size;
        *skip_size += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn knot_hash_str(string: &str) -> String {
        let hash = knot_hash(string);

        hash.iter().map(|b| format!("{:02x}", b)).join("")
    }

    #[test]
    fn test() {
        assert_eq!(knot_hash_str(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(
            knot_hash_str("AoC 2017"),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(knot_hash_str("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash_str("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
