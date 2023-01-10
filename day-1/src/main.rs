pub fn main() {
    let input = std::fs::read_to_string("day-1/input.txt").unwrap();
    let mut elves = vec![0];

    for line in input.lines() {
        if line.is_empty() {
            elves.push(0);
            continue;
        }

        *elves.last_mut().unwrap() += line.parse::<u32>().unwrap();
    }

    elves.sort();
    println!("Part one: {}", elves.last().unwrap());
    println!("Part two: {}", elves[elves.len() - 3..].iter().sum::<u32>());
}
