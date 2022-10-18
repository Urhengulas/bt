const KB: usize = 1024;

fn main() {
    let mut a = Vec::new();
    loop {
        push_one_mb(&mut a);
        println!("{} MB", a.len() / 1024)
    }
}

fn push_one_mb(a: &mut Vec<[u32; KB]>) {
    match a.try_reserve_exact(1024 * KB) {
        Ok(_) => {
            for _ in 0..(1024) {
                a.push([0; KB]);
            }
        }
        Err(e) => {
            dbg!(e);
            println!("Heap memory full. Handle it gracefully.");
            std::process::exit(0);
        }
    }
}
