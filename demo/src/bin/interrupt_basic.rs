#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_semihosting as _;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    #[cfg(feature = "enable-debug")]
    let _ = hprintln!("Demo is running");
    
    loop {}
}
