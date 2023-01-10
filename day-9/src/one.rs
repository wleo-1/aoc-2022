use aoc::HashSet;

pub fn run() {
    let mut grid = Grid {
        head: (0, 0),
        tail: (0, 0),
        visited: HashSet::from_iter([(0, 0)]),
    };

    move_all("day-9/input.txt", &mut grid);

    println!("Part one: {}", grid.visited.len());
}

struct Grid {
    head: (i32, i32),
    tail: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

fn move_once(grid: &mut Grid, dir: (i32, i32)) {
    grid.head.0 += dir.0;
    grid.head.1 += dir.1;

    match (grid.head.0 - grid.tail.0, grid.head.1 - grid.tail.1) {
        (-1..=1, -1..=1) => (),
        (x, y) => {
            grid.tail.0 += x.signum();
            grid.tail.1 += y.signum();
            grid.visited.insert(grid.tail);
        }
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

        for _ in 0..line.parse_int::<u32>() {
            move_once(grid, dir);
        }
    }
}
