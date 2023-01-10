pub fn run() {
    let input = std::fs::read_to_string("day-25/input.txt").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        sum += from_snafu(line);
    }

    let answer = to_snafu(sum);
    println!("Part one: {answer}");
}

fn from_snafu(snafu: &str) -> i64 {
    let snafu = snafu.chars().collect::<Vec<_>>();
    let mut num = 0;

    for (place, char) in snafu.into_iter().rev().enumerate() {
        let base = 5_i64.pow(place as u32);
        num += match char {
            '2' => 2 * base,
            '1' => base,
            '0' => 0,
            '-' => -base,
            '=' => -2 * base,
            _ => panic!(),
        };
    }

    num
}

fn to_snafu(mut num: i64) -> String {
    let mut snafu = Vec::new();

    while num != 0 {
        let rem = num % 5;
        num /= 5;

        let (char, carry) = match rem {
            0 => ("0", 0),
            1 => ("1", 0),
            2 => ("2", 0),
            3 => ("=", 1),
            4 => ("-", 1),
            _ => unreachable!(),
        };

        snafu.push(char);
        num += carry;
    }

    snafu.into_iter().rev().collect()
}
