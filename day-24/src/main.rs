use std::collections::HashSet;

mod one;
mod two;

fn main() {
    aoc::hello_world();

    one::run();
    two::run();
}

static MOVES: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug)]
pub struct Map {
    blizzards: Vec<Blizzard>,
    states: HashSet<(i32, i32)>,
    minutes: u32,
    goal: (i32, i32),
}

#[derive(Debug)]
struct Blizzard {
    pos: (i32, i32),
    dir: (i32, i32),
}

pub fn create_map(path: &'static str) -> Map {
    let input = std::fs::read_to_string(path).unwrap();

    let mut map = Map {
        blizzards: Vec::new(),
        states: HashSet::new(),
        minutes: 0,
        goal: (0, 0),
    };

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let dir = match char {
                '^' => (-1, 0),
                'v' => (1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                _ => continue,
            };

            let blizzard = Blizzard {
                pos: (row as i32, col as i32),
                dir,
            };

            map.blizzards.push(blizzard);
        }
    }

    map
}

pub fn dodge_blizzards(map: &mut Map, init: (i32, i32), goal: (i32, i32)) {
    map.states = HashSet::from([init]);
    map.goal = goal;
    while !do_tick(map) {}
}

fn do_tick(map: &mut Map) -> bool {
    map.minutes += 1;

    let at_goal = move_elves(map);
    move_blizzards_and_kill_elves(map);

    at_goal
}

// true if at goal
fn move_elves(map: &mut Map) -> bool {
    for (row, col) in map.states.clone() {
        for (row_offset, col_offset) in MOVES {
            match (row + row_offset, col + col_offset) {
                pos @ (1..=25, 1..=120) => {
                    map.states.insert(pos);
                }
                pos if pos == map.goal => return true,
                _ => (),
            }
        }
    }

    false
}

fn move_blizzards_and_kill_elves(map: &mut Map) {
    for blizzard in &mut map.blizzards {
        // move blizzards
        let (row, col) = blizzard.pos;
        let (row_offset, col_offset) = blizzard.dir;

        blizzard.pos = (row + row_offset, col + col_offset);

        match blizzard.pos {
            (0, _) => blizzard.pos.0 = 25,
            (26, _) => blizzard.pos.0 = 1,
            (_, 0) => blizzard.pos.1 = 120,
            (_, 121) => blizzard.pos.1 = 1,
            _ => (),
        }

        // kill elves
        map.states.remove(&blizzard.pos);
    }
}
