//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

//use panic_halt as _;
use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    let x = 10;
    hprintln!("Hello, world!").unwrap();

    assert_eq!(x, 8);

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
