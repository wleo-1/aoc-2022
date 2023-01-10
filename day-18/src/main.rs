use std::collections::HashSet;

mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

type Coords = [i32; 3];

const NEIGHBORS: [Coords; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

fn zip<T>([ax, ay, az]: Coords, [bx, by, bz]: Coords, op: impl Fn(i32, i32) -> T) -> [T; 3] {
    [op(ax, bx), op(ay, by), op(az, bz)]
}

fn create_droplet(path: &str) -> HashSet<Coords> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut droplet = HashSet::new();

    for line in input.lines() {
        let mut line = aoc::Input::new(line);

        let x = line.parse_int::<i32>();
        line.expect(",");
        let y = line.parse_int::<i32>();
        line.expect(",");
        let z = line.parse_int::<i32>();

        droplet.insert([x, y, z]);
    }

    droplet
}

fn calc_surface(shape: &HashSet<Coords>) -> u32 {
    let mut surface = 0;

    for &s in shape {
        for n in NEIGHBORS {
            if !shape.contains(&zip(s, n, |s, n| s + n)) {
                surface += 1;
            }
        }
    }

    surface
}
