use aoc::Itertools;
use std::collections::HashMap;

mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

#[derive(Debug)]
pub enum Item {
    Dir {
        parent: usize,
        files: HashMap<String, usize>,
    },
    File(u32),
}

fn new_item(fs: &mut Vec<Item>, item: Item) -> usize {
    let id = fs.len();
    fs.push(item);
    id
}

pub fn create_files() -> Vec<Item> {
    let input = std::fs::read_to_string("day-7/input.txt").unwrap();
    let mut lines = input.lines().peekable();

    let mut fs = Vec::new();

    let mut cd = new_item(
        &mut fs,
        Item::Dir {
            parent: 0,
            files: HashMap::new(),
        },
    );

    while let Some(line) = lines.next() {
        let mut chars = aoc::Input::new(line);

        chars.skip(2); // "$ "
        let cmd = chars.take(|char| char.is_ascii_alphabetic());

        match cmd.as_str() {
            "cd" => {
                chars.skip(1); // " "

                cd = match chars.take_all().as_str() {
                    "/" => 0,
                    ".." => {
                        let Item::Dir { parent, .. } = fs[cd] else {
                            panic!();
                        };

                        parent
                    }
                    dest => {
                        let Item::Dir { ref mut files, .. } = fs[cd] else {
                            panic!();
                        };

                        match files.get(dest) {
                            Some(id) => *id,
                            None => new_item(
                                &mut fs,
                                Item::Dir {
                                    parent: cd,
                                    files: HashMap::new(),
                                },
                            ),
                        }
                    }
                };
            }
            "ls" => {
                let lines = lines.peeking_take_while(|line| line.chars().next().unwrap() != '$');

                for line in lines {
                    let mut chars = aoc::Input::new(line);

                    let kind = chars.take(|char| char.is_ascii_alphanumeric());
                    chars.skip(1); // " "
                    let name = chars.take_all();

                    let Item::Dir { ref mut files, .. } = fs[cd] else {
                        panic!();
                    };

                    if files.get(&name).is_none() {
                        let item = match kind.as_str() {
                            "dir" => Item::Dir {
                                parent: cd,
                                files: HashMap::new(),
                            },
                            size => Item::File(size.parse::<u32>().unwrap()),
                        };

                        let id = new_item(&mut fs, item);

                        let Item::Dir { ref mut files, .. } = fs[cd] else {
                            panic!();
                        };
                        files.insert(name, id);
                    }
                }
            }
            _ => panic!(),
        }
    }

    fs
}
