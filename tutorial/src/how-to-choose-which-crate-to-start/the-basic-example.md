#### <a name="the-basic-example">4.1 The basic example</a>

- Make sure your `Cargo.toml` has the settings below:

    The `[features]` config tells the compiler to know that which feature rely on the particular optional dependencies.

    ```rust
    [features]
    default = []
    enable-debug = ["cortex-m-semihosting"]
    enable-pac = ["stm32f4"]
    enable-hal = ["stm32f4xx-hal"]
    
    [dependencies]
    cortex-m = "0.6.0"
    cortex-m-rt = "0.6.10"
    # panic-halt = "0.2.0"
    
    # For debugging purpose, enable `exit` feature
    panic-semihosting = { version = "0.5.3", features = ['exit'] }

    # Print debug info to host console, optional
    cortex-m-semihosting = { version = "0.3.3", optional = true }

    # PAC (Peripheral Access Crate), optional
    stm32f4 = { version = "0.11.0", features = ["stm32f407", "rt"], optional = true }

    # HAL (Hardware Abstraction Layer), optional
    stm32f4xx-hal = { version = "0.8.3", features = ['stm32f407'], optional = true }
    ```

- Make sure your `.cargo/config` has the settings below:

    ```rust
    [target.thumbv7em-none-eabi]
    # Settings below will make `cargo run` execute programs on QEMU
    # Normal version
    runner = "qemu-system-gnuarmeclipse -cpu cortex-m4 -mcu STM32F407VG -machine STM32F4-Discovery -nographic -semihosting-config enable=on,target=native -kernel"
    ```


- Create [`demo/src/bin/basic.rs`](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/bin/basic.rs) with the following code:

    ```rust
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
    ```

- How to run

    ```rust
    # cd demo
    cargo watch -c --exec 'run --bin basic --features "enable-debug"'
    ```
    Every time you save `demo/src/bin/basic.rs`, `cargo run --bin basic` will run again.

    ```rust
    (qemu) System timer demo is running >>>>>
    x is 10
    [examples/basic.rs:35] x = 10
    panicked at 'assertion failed: `(left == right)`
    left: `10`,
    right: `8`', examples/basic.rs:36:5
    [Finished running. Exit status: 1]
    ```
