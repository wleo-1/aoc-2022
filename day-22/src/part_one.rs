use aoc::Itertools;

pub fn run() {
    let mut map = load_map();
    let instr_txt = std::fs::read_to_string("day-22/instructions.txt").unwrap();
    let mut instr = instr_txt.chars().peekable();

    fn go_forward(map: &mut Map) {
        match map.facing {
            0 => go_right(map),
            1 => go_down(map),
            2 => go_left(map),
            3 => go_up(map),
            _ => panic!(),
        }
    }

    fn turn_right(map: &mut Map) {
        map.facing += 1;
        map.facing %= 4;
    }

    fn turn_left(map: &mut Map) {
        map.facing += 3;
        map.facing %= 4;
    }

    while let Some(char) = instr.peek() {
        match char {
            '0'..='9' => {
                let num = instr
                    .peeking_take_while(|char| char.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                for _ in 0..num {
                    go_forward(&mut map);
                }
            }
            'R' => {
                instr.next();
                turn_right(&mut map);
            }
            'L' => {
                instr.next();
                turn_left(&mut map);
            }
            _ => panic!(),
        }
    }

    let answer = (map.curr_row + 1) * 1000 + (map.curr_col + 1) * 4 + map.facing as usize;
    println!(
        "Part one: {answer}, pos: ({}, {}), facing: {}",
        map.curr_col, map.curr_row, map.facing
    );
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

fn load_map() -> Map {
    let map_txt = std::fs::read_to_string("day-22/map.txt").unwrap();

    // starting pos is (0, 50)
    let mut map = Map {
        rows: Vec::new(),
        curr_row: 0,
        curr_col: 50,
        facing: 0,
    };

    for line in map_txt.lines() {
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

    map
}

fn go_right(map: &mut Map) {
    let row = &map.rows[map.curr_row];

    match row.tiles.get(map.curr_col - row.offset + 1) {
        Some(Tile::Empty) => map.curr_col += 1,
        Some(Tile::Wall) => (),
        None => match row.tiles.first().unwrap() {
            Tile::Empty => map.curr_col = row.offset,
            Tile::Wall => (),
        },
    }
}

fn go_left(map: &mut Map) {
    let row = &map.rows[map.curr_row];

    match map
        .curr_col
        .checked_sub(row.offset + 1)
        .and_then(|idx| row.tiles.get(idx))
    {
        Some(Tile::Empty) => map.curr_col -= 1,
        Some(Tile::Wall) => (),
        None => match row.tiles.last().unwrap() {
            Tile::Empty => map.curr_col = row.offset + row.tiles.len() - 1,
            Tile::Wall => (),
        },
    }
}

fn go_down(map: &mut Map) {
    if let Some(row) = map.rows.get(map.curr_row + 1) {
        if map.curr_col >= row.offset && map.curr_col < row.offset + row.tiles.len() {
            match row.tiles[map.curr_col - row.offset] {
                Tile::Empty => map.curr_row += 1,
                Tile::Wall => (),
            }

            return;
        }
    }

    let return_row = map.curr_row;

    loop {
        if let Some(row) = map
            .curr_row
            .checked_sub(1)
            .and_then(|idx| map.rows.get(idx))
        {
            if map.curr_col >= row.offset && map.curr_col < row.offset + row.tiles.len() {
                map.curr_row -= 1;
                continue;
            }
        }

        let row = &map.rows[map.curr_row];
        match row.tiles[map.curr_col - row.offset] {
            Tile::Empty => (),
            Tile::Wall => map.curr_row = return_row,
        }

        break;
    }
}

fn go_up(map: &mut Map) {
    if let Some(row) = map
        .curr_row
        .checked_sub(1)
        .and_then(|idx| map.rows.get(idx))
    {
        if map.curr_col >= row.offset && map.curr_col < row.offset + row.tiles.len() {
            match row.tiles[map.curr_col - row.offset] {
                Tile::Empty => map.curr_row -= 1,
                Tile::Wall => (),
            }

            return;
        }
    }

    let return_row = map.curr_row;

    loop {
        if let Some(row) = map.rows.get(map.curr_row + 1) {
            if map.curr_col >= row.offset && map.curr_col < row.offset + row.tiles.len() {
                map.curr_row += 1;
                continue;
            }
        }

        let row = &map.rows[map.curr_row];
        match row.tiles[map.curr_col - row.offset] {
            Tile::Empty => (),
            Tile::Wall => map.curr_row = return_row,
        }

        break;
    }
}
