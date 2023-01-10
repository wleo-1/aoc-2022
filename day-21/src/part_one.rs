use std::collections::HashMap;

enum Monkey {
    Num(f64),
    Op {
        op: Operation,
        lhs: String,
        rhs: String,
    },
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn run() {
    let monkeys = create_monkeys();
    let answer = resolve_monkey(&monkeys, "root");

    println!("Part one: {answer}");
}

fn create_monkeys() -> HashMap<String, Monkey> {
    let input = std::fs::read_to_string("day-21/input.txt").expect("file");
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let mut iter = line.chars().peekable();

        let name = iter.by_ref().take(4).collect::<String>();

        iter.next();
        iter.next();

        let monkey = match iter.peek().expect("num or op") {
            '0'..='9' => {
                let num = iter.collect::<String>().parse().expect("number");
                Monkey::Num(num)
            }
            'a'..='z' => {
                let lhs = iter.by_ref().take(4).collect::<String>();
                iter.next();
                let op = iter.next().unwrap();
                iter.next();
                let rhs = iter.by_ref().take(4).collect::<String>();

                let op = match op {
                    '+' => Operation::Add,
                    '-' => Operation::Sub,
                    '*' => Operation::Mul,
                    '/' => Operation::Div,
                    _ => panic!("expected op"),
                };

                Monkey::Op { op, lhs, rhs }
            }
            _ => panic!("expected num or op"),
        };

        monkeys.insert(name, monkey);
    }

    monkeys
}

fn resolve_monkey(monkeys: &HashMap<String, Monkey>, name: &str) -> f64 {
    let monkey = monkeys.get(name).expect("fake monkey wtf");

    match monkey {
        Monkey::Num(num) => *num,
        Monkey::Op { op, lhs, rhs } => {
            let lhs = resolve_monkey(monkeys, &lhs);
            let rhs = resolve_monkey(monkeys, &rhs);

            match op {
                Operation::Add => lhs + rhs,
                Operation::Sub => lhs - rhs,
                Operation::Mul => lhs * rhs,
                Operation::Div => lhs / rhs,
            }
        }
    }
}
