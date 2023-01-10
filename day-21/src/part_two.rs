use std::collections::HashMap;

// each monkey's value is only used once, i think
#[derive(Debug)]
struct Monkey {
    parent: Option<String>,
    op: Operand,
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    Unknown,
    Human,
    Num(f64),
    Op {
        op: Operation,
        lhs: String,
        rhs: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operand {
    Unknown,
    Lhs,
    Rhs,
}

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

pub fn run() {
    let monkeys = create_monkeys();
    let human = monkeys.get("humn").expect("no humans?");
    let answer = resolve_up(
        &monkeys,
        &human.parent.as_ref().expect("fake human parent"),
        human.op,
    );

    println!("Part two: {answer}");
}

fn create_monkeys() -> HashMap<String, Monkey> {
    let input = std::fs::read_to_string("day-21/input.txt").expect("file");
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let mut iter = line.chars().peekable();

        let name = iter.by_ref().take(4).collect::<String>();

        let kind = if let "humn" = name.as_str() {
            Kind::Human
        } else {
            iter.next();
            iter.next();

            match iter.peek().expect("num or op") {
                '0'..='9' => {
                    let num = iter.collect::<String>().parse().expect("number");
                    Kind::Num(num)
                }
                'a'..='z' => {
                    let lhs = iter.by_ref().take(4).collect::<String>();
                    iter.next();
                    let op = iter.next().unwrap();
                    iter.next();
                    let rhs = iter.by_ref().take(4).collect::<String>();

                    insert_operand_monkey(&mut monkeys, &name, &lhs, Operand::Lhs);
                    insert_operand_monkey(&mut monkeys, &name, &rhs, Operand::Rhs);

                    let op = if let "root" = name.as_str() {
                        Operation::Eq
                    } else {
                        match op {
                            '+' => Operation::Add,
                            '-' => Operation::Sub,
                            '*' => Operation::Mul,
                            '/' => Operation::Div,
                            _ => panic!("expected op"),
                        }
                    };

                    Kind::Op { op, lhs, rhs }
                }
                _ => panic!("expected num or op"),
            }
        };

        if let Some(monkey) = monkeys.get_mut(&name) {
            monkey.kind = kind;
        } else {
            monkeys.insert(
                name,
                Monkey {
                    parent: None,
                    op: Operand::Unknown,
                    kind,
                },
            );
        }
    }

    monkeys
}

fn insert_operand_monkey(
    monkeys: &mut HashMap<String, Monkey>,
    parent: &str,
    op_name: &str,
    operand: Operand,
) {
    let parent = parent.to_string();

    if let Some(monkey) = monkeys.get_mut(op_name) {
        assert_eq!(monkey.parent, None);
        monkey.parent = Some(parent);
        monkey.op = operand;
    } else {
        monkeys.insert(
            op_name.to_string(),
            Monkey {
                parent: Some(parent),
                op: operand,
                kind: Kind::Unknown,
            },
        );
    }
}

fn resolve_down(monkeys: &HashMap<String, Monkey>, name: &str) -> f64 {
    let monkey = monkeys.get(name).expect("fake monkey wtf");

    match &monkey.kind {
        Kind::Unknown => panic!("literally fake monkey"),
        Kind::Human => panic!("human ?!"),
        Kind::Num(num) => *num,
        Kind::Op { op, lhs, rhs } => {
            let lhs = resolve_down(monkeys, &lhs);
            let rhs = resolve_down(monkeys, &rhs);

            match op {
                Operation::Add => lhs + rhs,
                Operation::Sub => lhs - rhs,
                Operation::Mul => lhs * rhs,
                Operation::Div => lhs / rhs,
                Operation::Eq => panic!("wow really"),
            }
        }
    }
}

fn resolve_up(monkeys: &HashMap<String, Monkey>, name: &str, operand: Operand) -> f64 {
    let monkey = monkeys.get(name).expect("fake monkey wtf");

    match &monkey.kind {
        Kind::Unknown => panic!("literally fake monkey"),
        Kind::Human => panic!("also human ?!"),
        Kind::Num(num) => panic!("fake {num} monkey"),
        Kind::Op { op, lhs, rhs } => {
            let val = if let Operation::Eq = op {
                None
            } else {
                let val = resolve_up(monkeys, &monkey.parent.as_ref().unwrap(), monkey.op);
                Some(val)
            };

            match operand {
                Operand::Unknown => panic!("fake operand"),
                Operand::Lhs => {
                    let rhs = resolve_down(monkeys, &rhs);

                    if let Some(val) = val {
                        match op {
                            Operation::Add => val - rhs,
                            Operation::Sub => val + rhs,
                            Operation::Mul => val / rhs,
                            Operation::Div => val * rhs,
                            Operation::Eq => unreachable!(),
                        }
                    } else {
                        rhs
                    }
                }
                Operand::Rhs => {
                    let lhs = resolve_down(monkeys, &lhs);

                    if let Some(val) = val {
                        match op {
                            Operation::Add => val - lhs,
                            Operation::Sub => lhs - val, // lhs - rhs = val => - rhs = val - lhs => rhs = lhs - val
                            Operation::Mul => val / lhs,
                            Operation::Div => lhs / val, // lhs / rhs = val => rhs / lhs = 1 / val => rhs = lhs / val
                            Operation::Eq => unreachable!(),
                        }
                    } else {
                        lhs
                    }
                }
            }
        }
    }
}
