#![no_main]
#![no_std]

use no_std_krate as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    no_std_krate::exit()
}
