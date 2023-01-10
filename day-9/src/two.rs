use aoc::HashSet;
use std::iter::repeat;

pub fn run() {
    let mut grid = Grid {
        head: (0, 0),
        body: repeat((0, 0)).take(8).collect(),
        tail: (0, 0),
        visited: HashSet::from_iter([(0, 0)]),
    };

    move_all("day-9/input.txt", &mut grid);

    println!("Part two: {}", grid.visited.len());
}

struct Grid {
    head: (i32, i32),
    body: Vec<(i32, i32)>,
    tail: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

fn move_once(head: (i32, i32), tail: (i32, i32)) -> Option<(i32, i32)> {
    match (head.0 - tail.0, head.1 - tail.1) {
        (-1..=1, -1..=1) => None,
        (x, y) => Some((tail.0 + x.signum(), tail.1 + y.signum())),
    }
}

fn move_all(path: &str, grid: &mut Grid) {
    let input = std::fs::read_to_string(path).unwrap();

    for line in input.lines() {
        let mut line = aoc::Input::new(line);

        let dir = match line.next().unwrap() {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!(),
        };

        line.skip(1);

        'outer: for _ in 0..line.parse_int::<u32>() {
            grid.head.0 += dir.0;
            grid.head.1 += dir.1;

            let mut head = grid.head;

            for node in grid.body.iter_mut() {
                match move_once(head, *node) {
                    Some(new) => {
                        *node = new;
                        head = new;
                    }
                    None => continue 'outer,
                }
            }

            if let Some(new) = move_once(head, grid.tail) {
                grid.tail = new;
                grid.visited.insert(new);
            }
        }
    }
}
