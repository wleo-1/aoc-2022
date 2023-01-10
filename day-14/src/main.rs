use std::collections::HashSet;

mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

const DIRS: [(i32, i32); 3] = [(0, 1), (-1, 1), (1, 1)];

#[derive(Debug)]
struct Cave {
    occupied: HashSet<(i32, i32)>,
    abyss: i32,
    count: usize,
}

fn create_cave(path: &str) -> Cave {
    let input = std::fs::read_to_string(path).unwrap();

    let mut cave = Cave {
        occupied: HashSet::new(),
        abyss: 0,
        count: 0,
    };

    for line in input.lines() {
        let mut line = aoc::Input::new(line);

        let x = line.parse_int::<i32>();
        line.expect(",");
        let y = line.parse_int::<i32>();

        let mut curr = (x, y);
        cave.occupied.insert(curr);
        cave.abyss = cave.abyss.max(y);

        while line.peek().is_some() {
            line.expect(" -> ");

            let x = line.parse_int::<i32>();
            line.expect(",");
            let y = line.parse_int::<i32>();

            cave.abyss = cave.abyss.max(y);

            let x_dif = (x - curr.0).signum();
            let y_dif = (y - curr.1).signum();

            while curr != (x, y) {
                curr.0 += x_dif;
                curr.1 += y_dif;

                cave.occupied.insert(curr);
            }
        }
    }

    cave
}
