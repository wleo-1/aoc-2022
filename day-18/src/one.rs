use crate::*;

pub fn run() {
    let droplet = create_droplet("day-18/input.txt");
    println!("Part one: {}", calc_surface(&droplet));
}
