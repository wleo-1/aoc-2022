use aoc::Itertools;
use std::collections::HashSet;

pub fn run() {
    let mut cube = create_map();

    let instr_txt = std::fs::read_to_string("day-22/instructions.txt").unwrap();
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
                    go_forward(&mut cube);
                }
            }
            'R' => {
                instr.next();
                turn_right(&mut cube);
            }
            'L' => {
                instr.next();
                turn_left(&mut cube);
            }
            _ => panic!(),
        }
    }

    // this turns out to be row 127, col 3, facing 0
    println!("Part one: pos: {:?}, facing: {:?}", cube.pos, cube.facing);
}

type Vector = [i32; 3];

// the intersection between sides 1, 3, 4 is (0, 0)
// x, y, z axes are normals to sides 1, 3, 4 respectively
// for rotation left, normal is is va and facing is vb
// for rotation right, swap the two
struct Cube {
    walls: HashSet<Vector>,
    norm: Vector,
    pos: Vector,
    facing: Vector,
}

fn create_map() -> Cube {
    let mut cube = Cube {
        walls: HashSet::new(),
        norm: [-1, 0, 0],
        pos: [0, 49, 0],
        facing: [0, 0, 1],
    };

    macro_rules! face {
        ($file:literal, $norm:expr, $fn:expr) => {
            create_face(&mut cube, concat!("day-22/cube/", $file), $norm, $fn)
        };
    }

    face!("1.txt", [-1, 0, 0], |row, col| [0, 49 - row, col]);
    face!("2.txt", [0, 0, 1], |row, col| [col, 49 - row, 49]);
    face!("3.txt", [0, -1, 0], |row, col| [row, 0, col]);
    face!("4.txt", [0, 0, -1], |row, col| [col, row, 0]);
    face!("5.txt", [1, 0, 0], |row, col| [49, row, col]);
    face!("6.txt", [0, 1, 0], |row, col| [col, 49, row]);

    cube
}

fn create_face(
    cube: &mut Cube,
    path: &'static str,
    norm: Vector,
    to_coords: impl Fn(i32, i32) -> Vector,
) {
    let str = std::fs::read_to_string(path).unwrap();

    for (row, line) in str.lines().enumerate() {
        let iter = line.chars().enumerate().filter(|(_, char)| *char == '#');

        for (col, _) in iter {
            cube.walls
                .insert(add(to_coords(row as i32, col as i32), norm));
        }
    }
}

fn add(va: Vector, vb: Vector) -> Vector {
    [va[0] + vb[0], va[1] + vb[1], va[2] + vb[2]]
}

fn cross(va: Vector, vb: Vector) -> Vector {
    [
        va[1] * vb[2] - va[2] * vb[1],
        va[2] * vb[0] - va[0] * vb[2],
        va[0] * vb[1] - va[1] * vb[0],
    ]
}

fn turn_left(cube: &mut Cube) {
    cube.facing = cross(cube.norm, cube.facing);
}

fn turn_right(cube: &mut Cube) {
    cube.facing = cross(cube.facing, cube.norm);
}

fn go_forward(cube: &mut Cube) {
    let mut new_pos = add(cube.pos, cube.facing);
    let mut new_facing = cube.facing;
    let mut new_norm = cube.norm;

    if new_pos.into_iter().any(|a| a < 0 || a > 49) {
        new_pos = cube.pos;
        new_facing = cross(cross(cube.norm, cube.facing), cube.facing);
        new_norm = cube.facing;
    }

    let wall_check = add(new_pos, new_norm);
    if !cube.walls.contains(&wall_check) {
        cube.pos = new_pos;
        cube.facing = new_facing;
        cube.norm = new_norm;
    }
}
