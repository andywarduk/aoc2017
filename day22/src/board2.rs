use std::collections::HashMap;

use crate::{dir::Dir, pos::Pos};

enum State {
    Weakened,
    Infected,
    Flagged,
}

pub struct Board2 {
    infected: HashMap<Pos, State>,
    vpos: Pos,
    vdir: Dir,
    infections: usize,
}

impl Board2 {
    pub fn new(input: &[Vec<bool>]) -> Self {
        let mut infected = HashMap::new();

        for (y, line) in input.iter().enumerate() {
            for (x, inf) in line.iter().enumerate() {
                if *inf {
                    infected.insert(Pos::new(x as isize, y as isize), State::Infected);
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
        match self.infected.get(&self.vpos) {
            Some(State::Weakened) => {
                *self.infected.get_mut(&self.vpos).unwrap() = State::Infected;
                self.infections += 1;
            }
            Some(State::Infected) => {
                self.turn_right();
                *self.infected.get_mut(&self.vpos).unwrap() = State::Flagged;
            }
            Some(State::Flagged) => {
                self.reverse();
                self.infected.remove(&self.vpos);
            }
            None => {
                self.turn_left();
                self.infected.insert(self.vpos.clone(), State::Weakened);
            }
        }

        self.forward()
    }

    fn turn_right(&mut self) {
        self.vdir = self.vdir.right();
    }

    fn turn_left(&mut self) {
        self.vdir = self.vdir.left();
    }

    fn reverse(&mut self) {
        self.vdir = self.vdir.reverse();
    }

    fn forward(&mut self) {
        self.vpos.move_dir(&self.vdir);
    }

    pub fn infections(&self) -> usize {
        self.infections
    }
}
