use std::{
    io::{self, Write},
    ptr,
};

fn main() {
    loop {
        // print "> "
        print!("> ");
        io::stdout().flush().unwrap();

        // read input
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let s = buf.trim();

        // parse to usize
        let num = match usize::from_str_radix(s, 16) {
            Ok(num) => num,
            Err(_) => continue,
        };

        // read memory
        let m = unsafe { ptr::read(num as *const u8) };

        // print memory
        println!("{m:#X}");
    }
}
