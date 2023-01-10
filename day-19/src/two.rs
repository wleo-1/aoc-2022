use crate::*;
use aoc::rayon::prelude::*;

pub fn run() {
    let input = std::fs::read_to_string("day-19/input.txt").unwrap();
    let time = std::time::Instant::now();

    let product = input
        .lines()
        .take(3)
        .par_bridge()
        .map(|line| {
            let bp = create_blueprint(line);
            do_robots(0, &bp, 32, [0; 4], [1, 0, 0, 0], 0)
        })
        .product::<u32>();

    println!("Part two: {product}");
    println!("Elapsed: {:?}", time.elapsed());
}
