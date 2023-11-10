use crate::*;

pub fn run() {
    let (mut map, _) = create_map("day-23/input.txt");
    let mut round = 1;

    while do_round(&mut map) {
        round += 1;
    }

    println!("Part two: {round}");
}
