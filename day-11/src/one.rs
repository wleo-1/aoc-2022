use crate::*;

pub fn run() {
    println!("Part one: {}", calculate::<u32>("day-11/input.txt", 20));
}

impl Item for u32 {
    fn from(from: u32) -> Self {
        from
    }

    fn register(&mut self, _: &[u32]) {}

    fn add(&mut self, op: u32) {
        *self += op;
    }

    fn mul(&mut self, op: u32) {
        *self *= op;
    }

    fn square(&mut self) {
        *self *= *self;
    }

    fn reduce(&mut self) {
        *self /= 3;
    }

    fn divisible(&self, by: u32) -> bool {
        *self % by == 0
    }
}
