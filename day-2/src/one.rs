pub fn run() {
    let input = std::fs::read_to_string("day-2/input.txt").unwrap();

    let mut score = 0;

    for line in input.lines() {
        let mut chars = line.chars();

        let them = match chars.next().unwrap() {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => panic!(),
        };

        chars.next();

        let you = match chars.next().unwrap() {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!(),
        };

        score += you;

        if them == you {
            score += 3;
        }

        let req = match them + 1 {
            3 => 3,
            n => n % 3,
        };

        if req == you {
            score += 6;
        }
    }

    println!("Part one: {score}");
}
