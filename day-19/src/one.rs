use crate::*;
use aoc::rayon::prelude::*;

pub fn run() {
    let input = std::fs::read_to_string("day-19/input.txt").unwrap();
    let time = std::time::Instant::now();

    let sum = input
        .par_lines()
        .map(|line| {
            let bp = create_blueprint(line);
            let geodes = do_robots(0, &bp, 24, [0; 4], [1, 0, 0, 0], 0);
            bp.id * geodes
        })
        .sum::<u32>();

    println!("Part one: {sum}");
    println!("Elapsed: {:?}", time.elapsed());
}
