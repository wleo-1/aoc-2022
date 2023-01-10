use std::collections as col;

mod input;
mod parse;

pub use input::Input;
pub use itertools::Itertools;
pub use parse::Extract;
pub use rayon;
pub use tinyvec::*;

pub fn hello_world() {
    println!("Hello, world!");
}

pub type HashSet<T> = col::HashSet<T, col::hash_map::RandomState>;
