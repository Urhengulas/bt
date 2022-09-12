const KB: usize = 1024;

fn main() {
    let mut a = Vec::new();
    for i in 0..u128::MAX {
        push_one_mb(&mut a);
        println!("{i} MB")
    }
}

fn push_one_mb(a: &mut Vec<[u32; KB]>) {
    for _ in 0..(1024) {
        a.push([0; KB]);
    }
}
