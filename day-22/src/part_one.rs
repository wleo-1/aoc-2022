use aoc::Itertools;

pub fn run() {
    let (mut map, instr_txt) = load_map("day-22/input.txt");
    let mut instr = instr_txt.chars().peekable();

    while let Some(char) = instr.peek() {
        match char {
            '0'..='9' => {
                let num = instr
                    .peeking_take_while(|char| char.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                for _ in 0..num {
                    match map.facing {
                        0 => horizontal(&mut map, 1),
                        1 => vertical(&mut map, 1),
                        2 => horizontal(&mut map, -1),
                        3 => vertical(&mut map, -1),
                        _ => panic!(),
                    }
                }
            }
            'R' => {
                instr.next();

                map.facing += 1;
                map.facing %= 4;
            }
            'L' => {
                instr.next();

                map.facing += 3;
                map.facing %= 4;
            }
            _ => panic!(),
        }
    }

    let answer = (map.curr_row + 1) * 1000 + (map.curr_col + 1) * 4 + map.facing as usize;
    println!("Part one: {answer}");
}

// wrapping horizontally - easy
// wrapping vertically - iterate up until you find an empty

struct Map {
    rows: Vec<Row>,
    curr_row: usize,
    curr_col: usize,
    facing: u8,
}

struct Row {
    offset: usize,
    tiles: Vec<Tile>,
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Wall,
}

fn load_map(path: &str) -> (Map, String) {
    let map_txt = std::fs::read_to_string(path).unwrap();
    let mut lines = map_txt.lines();

    // starting pos is (0, 50)
    let mut map = Map {
        rows: Vec::new(),
        curr_row: 0,
        curr_col: 50,
        facing: 0,
    };

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut chars = line.chars();
        let mut row = Row {
            offset: chars.peeking_take_while(|char| *char == ' ').count(),
            tiles: Vec::new(),
        };

        for char in chars {
            let tile = match char {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                _ => panic!(),
            };

            row.tiles.push(tile);
        }

        map.rows.push(row);
    }

    (map, lines.next().unwrap().to_string())
}

fn horizontal(map: &mut Map, shift: isize) {
    let row = &map.rows[map.curr_row];
    let col = shift
        .wrapping_add_unsigned(map.curr_col - row.offset)
        .rem_euclid(row.tiles.len() as isize) as usize;

    if let Tile::Empty = row.tiles[col] {
        map.curr_col = row.offset + col;
    }
}

fn vertical(map: &mut Map, shift: isize) {
    fn get_row(map: &Map, curr_row: usize, shift: isize) -> Option<usize> {
        let idx = curr_row.checked_add_signed(shift)?;
        let row = map.rows.get(idx)?;

        let min = map.curr_col >= row.offset;
        let max = map.curr_col < row.offset + row.tiles.len();
        (min && max).then_some(idx)
    }

    fn set_if_empty(map: &mut Map, idx: usize) {
        let row = &map.rows[idx];
        if let Tile::Empty = row.tiles[map.curr_col - row.offset] {
            map.curr_row = idx;
        }
    }

    if let Some(idx) = get_row(map, map.curr_row, shift) {
        set_if_empty(map, idx);
        return;
    }

    let mut curr_row = map.curr_row;
    while let Some(idx) = get_row(map, curr_row, -shift) {
        curr_row = idx;
    }

    set_if_empty(map, curr_row);
}
