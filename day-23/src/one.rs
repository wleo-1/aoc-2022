use crate::*;

pub fn run() {
    let (mut map, init_elves) = create_map("day-23/input.txt");

    for _ in 0..10 {
        do_round(&mut map);
    }

    let mut top = 0;
    let mut bottom = MAP_TOTAL - 1;
    let mut left = 0;
    let mut right = MAP_TOTAL - 1;

    while map.tiles[top].iter().all(|tile| *tile == Tile::Empty) {
        top += 1;
    }

    while map.tiles[bottom].iter().all(|tile| *tile == Tile::Empty) {
        bottom -= 1;
    }

    while map.tiles.iter().all(|row| row[left] == Tile::Empty) {
        left += 1;
    }

    while map.tiles.iter().all(|row| row[right] == Tile::Empty) {
        right -= 1;
    }

    let answer = (bottom - top + 1) * (right - left + 1) - init_elves;
    println!("Part one: {answer}");
}
