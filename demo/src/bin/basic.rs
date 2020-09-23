#![allow(warnings)]
// This attribute means the program won't use `std` crate which assumes
// an underlying OS. The program will use `core` crate, a subset of `std`
// that can run on bare metal systems.
#![no_std]
// This attribute means the program won't use the standard `main` interface.
#![no_main]

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

// `cortex_m_semihosting` gives us the ability to print the debug info
// into the host console. But keep in mind that, each write operation
// is super slow which takes several milliseconds depends on the
// hardware!!! That's why we make it as an option feature.
#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::{dbg, hprintln};

// `panic_semihosting` will call `debug::EXIT_FAILURE` after logging the
// panic message.
use panic_semihosting as _;

// We will use the `entry` attribute from the `cortex_m_rt` crate to define
// the entry point. The entry point function must have signature `fn() -> !;`
// which means can't return, as the program never terminates.
#[entry]
fn main() -> ! {
    // `hprintln` returns `Result<(),()>`
    #[cfg(feature = "enable-debug")]
    let _ = hprintln!("Basic STM32F4 demo is running >>>>>");

    // Get the singleton `Peripherals` instance. This method can only
    // successfully called **once()**, that's why return an `Option`.
    let peripherals = Peripherals::take().unwrap();

    // You can't do this, as `cortex_m::Peripherals` cannot be formatted
    // using `{:?}` because it doesn't implement `core::fmt::Debug`.
    // dbg!(peripherals);

    let x = 10;
    #[cfg(feature = "enable-debug")]
    {
        hprintln!("x is {}", x);
        dbg!(x);
    }

    // This will panic!!!
    assert_eq!(x, 8);

    loop {
        // Your program loop code here
    }
}
