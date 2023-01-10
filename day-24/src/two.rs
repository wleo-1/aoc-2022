use crate::*;

pub fn run() {
    let mut map = create_map("day-24/input.txt");

    dodge_blizzards(&mut map, (0, 1), (26, 120));
    dodge_blizzards(&mut map, (26, 120), (0, 1));
    dodge_blizzards(&mut map, (0, 1), (26, 120));

    println!("Part two: {}", map.minutes);
}
