mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

pub trait Item {
    fn from(from: u32) -> Self;
    fn register(&mut self, tests: &[u32]);

    fn add(&mut self, op: u32);
    fn mul(&mut self, op: u32);
    fn square(&mut self);

    fn reduce(&mut self);
    fn divisible(&self, by: u32) -> bool;
}

pub fn calculate<T: Item>(path: &str, rounds: usize) -> usize {
    let mut state = create_monkeys::<T>(path);

    for _ in 0..rounds {
        do_round::<T>(&mut state);
    }

    state.monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    state.monkeys[0].inspected * state.monkeys[1].inspected
}

struct State<T> {
    monkeys: Vec<Monkey<T>>,
}

struct Monkey<T> {
    items: Vec<T>,
    op: Op,
    test: u32,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

enum Op {
    Add(u32),
    Mul(u32),
    Square,
}

fn create_monkeys<T: Item>(path: &str) -> State<T> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut input = input.chars().peekable();

    let template = "\
Monkey {id:n}:
  Starting items: {items:n,}
  Operation: new = {lhs:s} {op:c} {rhs:s}
  Test: divisible by {div:n}
    If true: throw to monkey {if_true:n}
    If false: throw to monkey {if_false:n}";

    let mut extr = aoc::Extract::compile(template);

    let mut state = State::<T> {
        monkeys: Vec::new(),
    };

    let mut tests = Vec::new();

    while let Some(char) = input.peek() {
        if *char == '\n' {
            input.next();
            continue;
        }

        extr.parse(&mut input);

        let lhs = extr.get::<String>("lhs");
        let op = extr.get::<char>("op");
        let rhs = extr.get::<String>("rhs");

        let op = match (lhs.as_str(), op, rhs.as_str()) {
            ("old", '+', "old") => Op::Mul(2),
            ("old", '*', "old") => Op::Square,
            ("old", '+', op) | (op, '+', "old") => Op::Add(op.parse().unwrap()),
            ("old", '*', op) | (op, '*', "old") => Op::Mul(op.parse().unwrap()),
            _ => panic!(),
        };

        let items = extr
            .get_list::<u32>("items")
            .into_iter()
            .map(|item| Item::from(item))
            .collect();

        let test = extr.get::<u32>("div");
        tests.push(test);

        state.monkeys.push(Monkey {
            items,
            op,
            test,
            if_true: extr.get::<usize>("if_true"),
            if_false: extr.get::<usize>("if_false"),
            inspected: 0,
        });
    }

    for monkey in &mut state.monkeys {
        for item in &mut monkey.items {
            item.register(&tests);
        }
    }

    state
}

fn do_monkey<T: Item>(state: &mut State<T>, id: usize) {
    for mut item in std::mem::take(&mut state.monkeys[id].items) {
        let monkey = &mut state.monkeys[id];
        monkey.inspected += 1;

        match monkey.op {
            Op::Add(op) => item.add(op),
            Op::Mul(op) => item.mul(op),
            Op::Square => item.square(),
        }

        item.reduce();

        let to = if item.divisible(monkey.test) {
            monkey.if_true
        } else {
            monkey.if_false
        };

        state.monkeys[to].items.push(item);
    }
}

fn do_round<T: Item>(state: &mut State<T>) {
    for id in 0..state.monkeys.len() {
        do_monkey::<T>(state, id);
    }
}
