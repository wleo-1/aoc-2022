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

        let inc = match chars.next().unwrap() {
            'X' => match them + 2 {
                3 => 3,
                n => n % 3,
            },
            'Y' => them + 3,
            'Z' => {
                let req = match them + 1 {
                    3 => 3,
                    n => n % 3,
                };

                req + 6
            }
            _ => panic!(),
        };

        score += inc;
    }

    println!("Part one: {score}");
}
