use crate::*;

pub fn run() {
    let fs = create_files();
    let mut total = 0;

    calculate(&fs, 0, &mut total);

    println!("Part one: {total}");
}

fn calculate(fs: &Vec<Item>, id: usize, total: &mut u32) -> u32 {
    match &fs[id] {
        Item::Dir { files, .. } => {
            let size = files.values().map(|id| calculate(fs, *id, total)).sum();

            if size <= 100000 {
                *total += size;
            }

            size
        }
        Item::File(size) => *size,
    }
}
