use std::collections::HashSet;

use crate::{dir::Dir, pos::Pos};

pub struct Board1 {
    infected: HashSet<Pos>,
    vpos: Pos,
    vdir: Dir,
    infections: usize,
}

impl Board1 {
    pub fn new(input: &[Vec<bool>]) -> Self {
        let mut infected = HashSet::new();

        for (y, line) in input.iter().enumerate() {
            for (x, inf) in line.iter().enumerate() {
                if *inf {
                    infected.insert(Pos::new(x as isize, y as isize));
                }
            }
        }

        let vx = (input[0].len() / 2) as isize;
        let vy = (input.len() / 2) as isize;

        Self {
            infected,
            vpos: Pos::new(vx, vy),
            vdir: Dir::N,
            infections: 0,
        }
    }

    pub fn steps(&mut self, count: usize) {
        for _ in 0..count {
            self.step()
        }
    }

    fn step(&mut self) {
        if self.infected.contains(&self.vpos) {
            self.turn_right();
            self.infected.remove(&self.vpos);
        } else {
            self.turn_left();
            self.infected.insert(self.vpos.clone());
            self.infections += 1;
        }

        self.forward()
    }

    fn turn_right(&mut self) {
        self.vdir = self.vdir.right();
    }

    fn turn_left(&mut self) {
        self.vdir = self.vdir.left();
    }

    fn forward(&mut self) {
        self.vpos.move_dir(&self.vdir);
    }

    pub fn infections(&self) -> usize {
        self.infections
    }
}
