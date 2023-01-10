use std::collections::{HashMap, HashSet};

mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

#[derive(Debug)]
struct Grid {
    sensors: HashMap<(i64, i64), u64>, // radius
    beacons: HashSet<(i64, i64)>,
}

fn create_grid(path: &str) -> Grid {
    let input = std::fs::read_to_string(path).unwrap();

    let pattern = "Sensor at x={sx:n}, y={sy:n}: closest beacon is at x={bx:n}, y={by:n}";
    let mut extr = aoc::Extract::compile(pattern);

    let mut grid = Grid {
        sensors: HashMap::new(),
        beacons: HashSet::new(),
    };

    for line in input.lines() {
        extr.parse_str(line);

        let sx = extr.get::<i64>("sx");
        let sy = extr.get::<i64>("sy");
        let bx = extr.get::<i64>("bx");
        let by = extr.get::<i64>("by");

        let manhattan = sx.abs_diff(bx) + sy.abs_diff(by);
        grid.sensors.insert((sx, sy), manhattan);
        grid.beacons.insert((bx, by));
    }

    grid
}
