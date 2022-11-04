const KB: usize = 1024;

fn main() {
    let mut a = Vec::new();
    loop {
        push_one_kb(&mut a);
    }
}

fn push_one_kb(a: &mut Vec<[u8; KB]>) {
    a.push([0; KB]);
}
