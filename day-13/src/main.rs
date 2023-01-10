mod one;
mod two;

pub use std::cmp::Ordering;

fn main() {
    one::run();
    two::run();
}

fn calculate(a: &str, b: &str) -> Ordering {
    compare(&mut aoc::Input::new(a), &mut aoc::Input::new(b))
}

fn compare(a: &mut aoc::Input, b: &mut aoc::Input) -> Ordering {
    macro_rules! cmp {
        ($a:expr, $b:expr) => {
            match compare($a, $b) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => (),
            }
        };
    }

    macro_rules! list_term {
        ($a:expr, $b:expr) => {
            match ($a.peek().unwrap(), $b.peek().unwrap()) {
                (']', ']') => {
                    $a.next();
                    $b.next();

                    return Ordering::Equal;
                }
                (']', _) => return Ordering::Less,
                (_, ']') => return Ordering::Greater,
                _ => (),
            }
        };
    }

    match (a.peek().unwrap(), b.peek().unwrap()) {
        ('[', '[') => {
            a.next();
            b.next();

            list_term!(a, b);

            loop {
                cmp!(a, b);
                list_term!(a, b);
                a.next();
                b.next();
            }
        }
        ('[', _) => {
            a.next();

            if a.peek().unwrap() == ']' {
                return Ordering::Less;
            }

            cmp!(a, b);

            if a.next().unwrap() == ']' {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
        (_, '[') => {
            b.next();

            if b.peek().unwrap() == ']' {
                return Ordering::Greater;
            }

            cmp!(a, b);

            if b.next().unwrap() == ']' {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        }
        _ => {
            let a = a.parse_int::<u32>();
            let b = b.parse_int::<u32>();

            match a.cmp(&b) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            }
        }
    }
}
