use std::{fs, path::PathBuf};

const DIRS: [&str; 2] = ["yes_std", "no_std"];

fn main() {
    for dir in DIRS {
        let p = PathBuf::from(format!("../{dir}/src/bin"));
        println!("# {dir}");
        print_dir(p);
    }
}

fn print_dir(dir: PathBuf) {
    let files = dir
        .read_dir()
        .unwrap()
        .filter_map(|entry| entry.map(|e| e.path()).ok())
        .collect::<Vec<_>>();
    for f in files.into_iter() {
        let file_name = f.file_name().unwrap().to_string_lossy();
        let content = fs::read_to_string(&f).unwrap();
        println!("## {file_name}\n\n```rust\n{}\n```\n", content)
    }
}
