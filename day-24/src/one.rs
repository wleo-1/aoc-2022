use crate::*;

pub fn run() {
    let mut map = create_map("day-24/input.txt");

    dodge_blizzards(&mut map, (0, 1), (26, 120));

    println!("Part one: {}", map.minutes);
}
