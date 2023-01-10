use crate::*;

pub fn run() {
    let input = std::fs::read_to_string("day-13/input.txt").unwrap();

    let mut packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    packets.push("[[2]]");
    packets.push("[[6]]");
    packets.sort_by(|a, b| calculate(a, b));

    let mut answer = 1;

    for (idx, packet) in packets.into_iter().enumerate() {
        match packet {
            "[[2]]" => answer *= idx + 1,
            "[[6]]" => {
                answer *= idx + 1;
                break;
            }
            _ => continue,
        }
    }

    println!("Part two: {answer}");
}
