const KB: usize = 1024;

fn main() {
    let mut a = Vec::new();
    loop {
        push_one_mb(&mut a);
        println!("{} MB", a.len() / 1024)
    }
}

fn push_one_mb(a: &mut Vec<[u32; KB]>) {
    for _ in 0..(1024) {
        a.push([0; KB]);
    }
}
