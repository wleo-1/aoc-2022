use crate::*;
use std::collections::HashMap;

pub fn run() {
    let answer = calculate::<UnbridledAngst>("day-11/input.txt", 10_000);
    println!("Part two: {answer}");
}

struct UnbridledAngst {
    init: u32,
    factors: HashMap<u32, u32>,
}

impl Item for UnbridledAngst {
    fn from(from: u32) -> Self {
        Self {
            init: from,
            factors: HashMap::new(),
        }
    }

    fn register(&mut self, tests: &[u32]) {
        let iter = tests
            .iter()
            .copied()
            .map(|factor| (factor, self.init % factor));

        self.factors.extend(iter);
    }

    fn add(&mut self, op: u32) {
        for rem in self.factors.values_mut() {
            *rem += op;
        }
    }

    fn mul(&mut self, op: u32) {
        for rem in self.factors.values_mut() {
            *rem *= op;
        }
    }

    fn square(&mut self) {
        for rem in self.factors.values_mut() {
            *rem *= *rem;
        }
    }

    fn reduce(&mut self) {
        for (factor, rem) in self.factors.iter_mut() {
            *rem %= *factor;
        }
    }

    fn divisible(&self, by: u32) -> bool {
        *self.factors.get(&by).unwrap() == 0
    }
}
