const KB: usize = 1024;

fn main() {
    let mut a = Vec::new();
    loop {
        push_one_kb(&mut a);
    }
}

fn push_one_kb(a: &mut Vec<[u8; KB]>) {
    match a.try_reserve_exact(1) {
        Ok(_) => {
            a.push([0; KB]);
        }
        Err(e) => {
            println!("AllocationError: {}.", e);
            println!("Handle allocation error gracefully.");
            std::process::exit(0);
        }
    }
}
