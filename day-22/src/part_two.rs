use aoc::Itertools;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let (mut cube, instr_txt) = create_map("day-22/input.txt", 50);
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

    let face = cube.faces.get(&cube.norm).unwrap();

    let r = face.rightwards.into_iter().take_while(|a| *a == 0).count();
    let d = cross(face.rightwards, cube.norm)
        .into_iter()
        .take_while(|a| *a == 0)
        .count();
    let row = cube.pos[d].abs_diff(face.corner[d]);
    let col = cube.pos[r].abs_diff(face.corner[r]);

    let row = face.pos.0 as u32 * cube.width + row + 1;
    let col = face.pos.1 as u32 * cube.width + col + 1;

    let mut rot = 0;
    let mut facing = face.rightwards;
    while facing != cube.facing {
        rot += 1;
        facing = cross(facing, cube.norm);
    }

    println!("Part two: {}", 1000 * row + 4 * col + rot);
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
    width: u32,
    faces: HashMap<Vector, Face>,
}

#[derive(Clone, Debug)]
struct Face {
    pos: (usize, usize),
    corner: Vector,
    rightwards: Vector,
}

struct Square {
    tiles: Vec<Vec<Tile>>,
}

enum Tile {
    Wall,
    Empty,
}

fn create_map(path: &str, width: u32) -> (Cube, String) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut input = input.lines().peekable();
    let mut faces = Vec::new();

    while let Some(line) = input.peek() {
        if line.is_empty() {
            input.next();
            break;
        }

        let mut arr = [None, None, None, None, None, None];

        for line in input.by_ref().take(width as usize) {
            let mut line = line.chars().peekable();

            for face in &mut arr {
                if line.peek().filter(|c| **c != ' ').is_none() {
                    line.by_ref().take(width as usize).for_each(drop);
                    continue;
                }

                let face = face.get_or_insert(Square { tiles: Vec::new() });

                let mut vec = Vec::new();

                for char in line.by_ref().take(width as usize) {
                    let tile = match char {
                        '#' => Tile::Wall,
                        '.' => Tile::Empty,
                        _ => panic!(),
                    };

                    vec.push(tile);
                }

                face.tiles.push(vec);
            }
        }

        faces.push(arr);
    }

    let mut cube = Cube {
        walls: HashSet::new(),
        norm: [-1, 0, 0],
        pos: [0, width as i32 - 1, 0],
        facing: [0, 0, 1],
        width,
        faces: HashMap::new(),
    };

    let mut col = 0;
    for square in &faces[0] {
        if square.is_some() {
            break;
        }

        col += 1;
    }

    create_face(
        &mut cube,
        &mut faces,
        [-1, 0, 0],
        Face {
            pos: (0, col),
            corner: [0, width as i32 - 1, 0],
            rightwards: [0, 0, 1],
        },
    );

    (cube, input.next().unwrap().to_string())
}

fn create_face(cube: &mut Cube, faces: &mut Vec<[Option<Square>; 6]>, norm: Vector, face: Face) {
    let square = faces[face.pos.0][face.pos.1].take().unwrap();

    let mut pos = face.corner;
    let downwards = cross(face.rightwards, norm);
    let carriage_return = scale(face.rightwards, -(cube.width as i32));

    cube.faces.insert(norm, face.clone());

    for row in square.tiles {
        for tile in row {
            if let Tile::Wall = tile {
                cube.walls.insert(add(pos, norm));
            }

            pos = add(pos, face.rightwards);
        }

        pos = add(pos, downwards);
        pos = add(pos, carriage_return);
    }

    const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn check_neighbor(
        faces: &Vec<[Option<Square>; 6]>,
        (row, col): (usize, usize),
        (srow, scol): (isize, isize),
    ) -> Option<(usize, usize)> {
        let (row, col) = (row.checked_add_signed(srow)?, col.checked_add_signed(scol)?);
        faces.get(row)?.get(col)?.as_ref()?;
        Some((row, col))
    }

    for (rot, neighbor) in NEIGHBORS.into_iter().enumerate() {
        let Some((row, col)) = check_neighbor(faces, face.pos, neighbor) else {
            continue;
        };

        let backwards = scale(norm, -1);

        let (norm, corner_shift, rightwards) = match rot {
            0 => (face.rightwards, face.rightwards, backwards),
            1 => (downwards, downwards, face.rightwards),
            2 => (scale(face.rightwards, -1), backwards, norm),
            3 => (scale(downwards, -1), backwards, face.rightwards),
            _ => panic!(),
        };

        create_face(
            cube,
            faces,
            norm,
            Face {
                pos: (row, col),
                corner: add(face.corner, scale(corner_shift, cube.width as i32 - 1)),
                rightwards,
            },
        );
    }
}

fn add(va: Vector, vb: Vector) -> Vector {
    [va[0] + vb[0], va[1] + vb[1], va[2] + vb[2]]
}

fn scale(v: Vector, n: i32) -> Vector {
    v.map(|a| a * n)
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

    if new_pos
        .into_iter()
        .any(|a| a < 0 || a > cube.width as i32 - 1)
    {
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
