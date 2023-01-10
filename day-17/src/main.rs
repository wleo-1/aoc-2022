use aoc::{array_vec, ArrayVec};
use std::collections::HashMap;

mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

struct Grid {
    tiles: Vec<[bool; 7]>,
    jets: Vec<isize>,
    curr_tile: u8,
    curr_jet: usize,
}

// offset from bottom left corner
fn kind_tiles(kind: u8) -> ArrayVec<[(usize, usize); 5]> {
    match kind {
        0 => array_vec!([(usize, usize); 5] => (0, 0), (0, 1), (0, 2), (0, 3)),
        1 => array_vec!([(usize, usize); 5] => (0, 1), (1, 0), (1, 1), (1, 2), (2, 1)),
        2 => array_vec!([(usize, usize); 5] => (0, 0), (0, 1), (0, 2), (1, 2), (2, 2)),
        3 => array_vec!([(usize, usize); 5] => (0, 0), (1, 0), (2, 0), (3, 0)),
        4 => array_vec!([(usize, usize); 5] => (0, 0), (0, 1), (1, 0), (1, 1)),
        _ => panic!(),
    }
}

fn valid_pos(
    grid: &Grid,
    kind_tiles: ArrayVec<[(usize, usize); 5]>,
    (row, col): (usize, usize),
) -> bool {
    kind_tiles.iter().cloned().all(|(row_offset, col_offset)| {
        let Some(row) = grid.tiles.get(row + row_offset) else {
            return col + col_offset < 7;
        };

        row.get(col + col_offset).filter(|tile| !*tile).is_some()
    })
}

fn do_rock(grid: &mut Grid) {
    let mut pos = (grid.tiles.len() + 3, 2_usize);
    let kind_tiles = kind_tiles(grid.curr_tile);
    grid.curr_tile += 1;
    grid.curr_tile %= 5;

    loop {
        let jet = grid.jets[grid.curr_jet];
        grid.curr_jet += 1;
        grid.curr_jet %= grid.jets.len();

        let new_pos = (pos.0, pos.1.saturating_add_signed(jet));
        if valid_pos(grid, kind_tiles, new_pos) {
            pos = new_pos;
        }

        if let Some(new_row) = pos.0.checked_sub(1) {
            let new_pos = (new_row, pos.1);
            if valid_pos(grid, kind_tiles, new_pos) {
                pos = new_pos;
                continue;
            }
        }

        for (row_offset, col_offset) in kind_tiles.iter().cloned() {
            while pos.0 + row_offset >= grid.tiles.len() {
                grid.tiles.push([false; 7]);
            }

            grid.tiles[pos.0 + row_offset][pos.1 + col_offset] = true;
        }

        break;
    }
}

#[derive(Debug)]
enum State {
    Once,
    Twice { rocks: u64, rows: usize },
}

fn calculate(path: &str, count: u64) -> u64 {
    let input = std::fs::read_to_string(path).unwrap();

    let mut grid = Grid {
        tiles: Vec::new(),
        jets: input
            .chars()
            .map(|char| match char {
                '<' => -1,
                '>' => 1,
                _ => panic!(),
            })
            .collect(),
        curr_tile: 0,
        curr_jet: 0,
    };

    let mut seen = HashMap::new();
    let mut skipped = None;

    let mut rocks = 0;
    while rocks < count {
        if skipped.is_none() {
            let key = (grid.curr_tile, grid.curr_jet);

            let new = match seen.get_mut(&key) {
                None => State::Once,
                Some(State::Once) => State::Twice {
                    rocks,
                    rows: grid.tiles.len(),
                },
                Some(State::Twice {
                    rocks: prev_rocks,
                    rows,
                }) => {
                    let diff = rocks - *prev_rocks;
                    let inc = (grid.tiles.len() - *rows) as u64;

                    let skipped_rocks = (count - rocks) / diff;
                    rocks += skipped_rocks * diff;
                    skipped = Some(skipped_rocks * inc);

                    continue;
                }
            };

            seen.insert(key, new);
        }

        do_rock(&mut grid);
        rocks += 1;
    }

    grid.tiles.len() as u64 + skipped.unwrap_or_default()
}
