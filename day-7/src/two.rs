use crate::*;

pub fn run() {
    let fs = create_files();
    let mut vec = Vec::new();

    let total = calculate(&fs, 0, &mut vec);
    let req = total - 40000000;
    let answer = vec.into_iter().filter(|size| *size >= req).min().unwrap();

    println!("Part two: {answer}");
}

fn calculate(fs: &Vec<Item>, id: usize, vec: &mut Vec<u32>) -> u32 {
    match &fs[id] {
        Item::Dir { files, .. } => {
            let size = files.values().map(|id| calculate(fs, *id, vec)).sum();
            vec.push(size);
            size
        }
        Item::File(size) => *size,
    }
}
