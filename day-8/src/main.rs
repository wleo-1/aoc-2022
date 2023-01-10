mod one;
mod two;

fn main() {
    one::run();
    two::run();
}

#[derive(Debug)]
struct Tree {
    height: u32,
    seen: bool,
}

pub struct Grid {
    visible: u32,
    trees: Vec<Vec<Tree>>,
}

pub fn create_grid(path: &str) -> Grid {
    let input = std::fs::read_to_string(path).unwrap();

    let mut grid = Grid {
        visible: 0,
        trees: Vec::new(),
    };

    for line in input.lines() {
        let mut row = Vec::new();

        for char in line.chars() {
            row.push(Tree {
                height: char.to_digit(10).unwrap() + 1,
                seen: false,
            });
        }

        grid.trees.push(row);
    }

    grid
}
