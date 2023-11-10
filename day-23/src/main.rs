use std::iter::{repeat, repeat_with};

mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

const MAP_WIDTH: usize = 70;
const MAP_PADDING: usize = 50;
const MAP_TOTAL: usize = MAP_WIDTH + 2 * MAP_PADDING;

static DIRS: [[(isize, isize); 3]; 4] = [
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, -1), (1, 0), (1, 1)],
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
];

static CIRCLE: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

// north, south, west, east
pub struct Map {
    dir: u8,
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Elf,
    Empty,
    Planned(usize, usize),
    Overpopulation,
}

pub fn create_map(path: &'static str) -> (Map, usize) {
    let input = std::fs::read_to_string(path).unwrap();
    let tiles = repeat_with(|| repeat(Tile::Empty).take(MAP_TOTAL).collect())
        .take(MAP_TOTAL)
        .collect();
    let mut map = Map { dir: 0, tiles };
    let mut init_elves = 0;

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if let '#' = char {
                map.tiles[row + MAP_PADDING][col + MAP_PADDING] = Tile::Elf;
                init_elves += 1;
            }
        }
    }

    (map, init_elves)
}

pub fn do_round(map: &mut Map) -> bool {
    first_half(map);
    let moved = second_half(map);

    map.dir += 1;
    map.dir %= 4;

    // clear planned moves from empty tiles
    for line in &mut map.tiles {
        for tile in line {
            if let Tile::Overpopulation = tile {
                *tile = Tile::Empty;
            }
        }
    }

    moved
}

fn first_half(map: &mut Map) {
    let map_dir = map.dir;

    for row in 0..map.tiles.len() {
        for col in 0..map.tiles[row].len() {
            let Tile::Elf = map.tiles[row][col] else {
                continue;
            };

            let pred = |offset| match neighbor(map, row, col, offset) {
                Some((row, col)) => map.tiles[row][col] != Tile::Elf,
                None => true,
            };

            if CIRCLE.into_iter().all(pred) {
                continue;
            }

            for dir in 0..4 {
                let dir = DIRS[(map_dir + dir) as usize % 4];

                if dir.into_iter().all(pred) {
                    if let Some((new_row, new_col)) = neighbor(map, row, col, dir[1]) {
                        let tile = &mut map.tiles[new_row][new_col];

                        match tile {
                            Tile::Empty => *tile = Tile::Planned(row, col),
                            Tile::Planned(..) => *tile = Tile::Overpopulation,
                            _ => (),
                        }

                        break;
                    }
                }
            }
        }
    }
}

fn neighbor(
    map: &Map,
    row: usize,
    col: usize,
    (row_offset, col_offset): (isize, isize),
) -> Option<(usize, usize)> {
    let row = row.checked_add_signed(row_offset)?;
    let col = col.checked_add_signed(col_offset)?;

    map.tiles.get(row)?.get(col)?;
    Some((row, col))
}

fn second_half(map: &mut Map) -> bool {
    let mut moved = false;

    for row in 0..map.tiles.len() {
        for col in 0..map.tiles[row].len() {
            let tile = &mut map.tiles[row][col];

            match *tile {
                Tile::Planned(row, col) => {
                    *tile = Tile::Elf;
                    map.tiles[row][col] = Tile::Empty;
                    moved = true;
                }
                _ => (),
            }
        }
    }

    moved
}
