use std::collections::VecDeque;

mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    queue: VecDeque<(usize, usize)>,
}

pub struct Tile {
    elevation: u32,
    best: Option<u32>,
    end: bool,
}

pub fn create_map(path: &str) -> Map {
    let input = std::fs::read_to_string(path).unwrap();

    let mut map = Map {
        tiles: Vec::new(),
        queue: VecDeque::new(),
    };

    for (row, line) in input.lines().enumerate() {
        let mut vec = Vec::new();

        for (col, char) in line.chars().enumerate() {
            let tile = match char {
                'S' => Tile {
                    elevation: 0,
                    best: None,
                    end: true,
                },
                'E' => {
                    map.queue.push_back((row, col));
                    Tile {
                        elevation: 26,
                        best: Some(0),
                        end: false,
                    }
                }
                'a'..='z' => Tile {
                    elevation: char as u32 - 97,
                    best: None,
                    end: false,
                },
                _ => panic!(),
            };

            vec.push(tile);
        }

        map.tiles.push(vec);
    }

    map
}

pub fn pathfind(map: &mut Map, criterion: impl Fn(&Tile) -> bool) -> u32 {
    loop {
        let (row, col) = map.queue.pop_front().unwrap();
        let tile = &map.tiles[row][col];
        let elevation = tile.elevation;
        let best = tile.best.unwrap() + 1;

        for (row_offset, col_offset) in NEIGHBORS {
            let row = row.saturating_add_signed(row_offset);
            let col = col.saturating_add_signed(col_offset);

            let neighbor = map
                .tiles
                .get_mut(row)
                .and_then(|line| line.get_mut(col))
                .filter(|neighbor| neighbor.elevation >= elevation.saturating_sub(1));

            if let Some(neighbor) = neighbor {
                if criterion(neighbor) {
                    return best;
                }

                if neighbor.best.is_none() {
                    neighbor.best = Some(best);
                    map.queue.push_back((row, col));
                }
            }
        }
    }
}
