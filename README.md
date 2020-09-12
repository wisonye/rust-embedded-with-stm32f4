# All-in-one Manual For `Rust` Embedded programming on `Mac`

In this manual, we will cover all the knowledge we need to know for using `Rust` to do embedded programming.

And we pick the `ARM-based MCU STM32F4` series chips as our target to run all the demos.

[**1. Important concepts**](#important-concepts)
- [_1.1 What is `MCU` and `Soc` ? What makes it different than `SBC`?_](#what-is-mcu)
- [_1.2 What is `STM32`?_](#what-is-stm32)
- [_1.3 The hardware we will use in the demo_](#the-hardware-we-will-use-in-the-demo)

[**2. Setup Environment**](#setup-environment)
- [_2.1 Install tooling_](#install-tooling)
- [_2.2 Create `demo` project from template_](#create-demo-project-from-template)
- [_2.3 Run hello example in `QEMU` and set break point in `ARM GDB`_](#run-hello-example-in-qemu)
- [_2.3.1 Run hello example in hardware_](#run-hello-example-in-hardware)
- [_2.3.2 Debugging in `vim` with the `ARM GDB`_](#debugging-in-vim-gdb)

[**3. How to choose which crate (rust's library) to start**](#how-to-choose-which-crate-to-start)
- [_3.1 The basic example_](#the-basic-example)
- [_3.2 The GPIO LED blinking example_](#the-gpio-led-example)

<hr>

## <a name="important-concepts">1. Important concepts</a>

#### <a name="what-is-mcu">1.1 What is `MCU` and `Soc` ? What makes it different than `SBC`?</a>

- `MCU` stands for `Microcontroller Unit`.
- `Soc` stands for `System-on-chip`.
- `SBC` stands for `Single Board Computer`.

Summaries, `MCU` and `Soc` sound like the same thing which is an all-in-one chip. It includes all the hardware
components below inside the single-chip:

- CPU (even FPU)
- RAM (Random Access Memory)
- ROM (Read-only Memory)
- Flash (as the hard drive to store your program)
- Any related I/O

It's basically a very small computer on an `IC` (integrated circuit) or microchip. Compare to `SBC`, it got a few different below:

- `MCU` usually with limited hardware power resources which suit for non-heavy computation solution.
- `MCU` can deal with fast power on and off at any given time and won't hurt your program which `SBC` can't do like that (as it got 
`OS` on it).
- `MCU` doesn't have `OS` at all, your program will be the **only** one program running there with full control and real-time responsiveness.

#### <a name="what-is-stm32">1.2 What is `STM32`?</a>

`STM32` is a family of **32-bit** `MCU` by **STMicroelectronics**. The `STM32` chips are grouped into related series that are based around the same **32-bit** ARM processor core, such as:

- **Cortex-M33F**
- **Cortex-M7F**
- **Cortex-M4F**
- **Cortex-M3**
- **Cortex-M0+**
- **Cortex-M0**

The `F` means with `FPU` (Floating Point Unit).

Internally, each `MCU` consists of the processor core, static RAM, flash memory, debugging interface, and various peripherals.

The company behind the `Arm` trademark (`Arm Holdings`) doesn't actually manufacture chips for purchase. Instead, their primary business model is to just design parts of chips. They will then license those designs to manufacturers, who will in turn implement the designs (perhaps with some of their own tweaks) in the form of physical hardware that can then be sold.


### <a name="the-hardware-we-will-use-in-the-demo">1.3 The hardware we will use in the demo</a>

**STM32F407G-DISC1** (replaces **STM32F4DISCOVERY**) dev board which with the `STM32F407VG` high performance `MCU`.

About the `STM32F407VG`:

High-performance foundation line, ARM **Cortex-M4** core with **DSP** and **FPU**, 1 Mbyte Flash, 168 MHz CPU, ART Accelerator, Ethernet, FSMC

More hardware details at [here](https://www.st.com/content/st_com/en/products/microcontrollers-microprocessors/stm32-32-bit-arm-cortex-mcus/stm32-high-performance-mcus/stm32f4-series/stm32f407-417/stm32f407vg.html), and the datasheet at [here](https://www.st.com/resource/en/datasheet/stm32f407vg.pdf).


## <a name="setup-environment">2. Setup environment</a>

Before we can write rust and test it on emulator or the real hardware, we need to install 
some tools below:

#### <a name="install-tooling">2.1 Install tooling</a>
- `rust` and related binaries
    
    ```bash
    # Install latest rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # `cargo-generate` used to generate the rust embedded project from template
    cargo install cargo-generate

    # `cargo-binutils` is a collection of cargo subcommand LLVM tools. Like:
    # `rust-objdump`,
    # `rust-readob`,
    # `rust-ld`,
    # `rust-lld`,
    # `rust-lld`,
    # `rust-nm`,
    # `rust-size`,
    # etc.
    cargo install cargo-binutils

    rustup component add llvm-tools-preview

    # We need this for hot-load
    cargo install cargo-watch

    # Add cross-compilation target
    rustup target add thumbv7em-none-eabi
    rustup target add thumbv7em-none-eabihf
    ```

- `QEMU` ARM emulator and `STM32F4` support

    ```bash
    brew install qemu
    ```

    `QEMU` is a generic and open source machine emulator. By default, it doesn't
    support `STM32F4`, below is the steps how to make that happen:

    ```bash
    # Detail is here: https://xpack.github.io/qemu-arm/install/
    cd ~/Downloads/
    curl -LJO "https://github.com/xpack-dev-tools/qemu-arm-xpack/releases/download/v2.8.0-9/xpack-qemu-arm-2.8.0-9-darwin-x64.tar.gz"
    mkdir -p ~/Library/xPacks/qemu-arm/2.8.0-9
    cd ~/Library/xPacks/qemu-arm/2.8.0-9
    tar xvf ~/Downloads/xpack-qemu-arm-2.8.0-9-darwin-x64.tar.gz
    mv xpack-qemu-arm-2.8.0-9/* ./
    rm -rf xpack-qemu-arm-2.8.0-9/
    chmod -R -w ./

    # Add `/Users/wison/Library/xPacks/qemu-arm/2.8.0-9/bin` to your `$PATH`
    # After that, test it:
    qemu-system-gnuarmeclipse --version

    # List all support machine and we should be able to see `STM32F4XXX`:
    qemu-system-gnuarmeclipse -machine help
    ```

- ARM version `GDB` and `OpenOCD` for remote debugging

    `GDB` isn't able to communicate directly with the ST-Link debugging hardware on your STM32F3DISCOVERY development board. It needs a translator and the Open On-Chip Debugger, `OpenOCD`, is that translator. `OpenOCD` is a program that runs on your laptop/PC and translates between GDB's TCP/IP based remote debug protocol and ST-Link's USB based protocol.

    `OpenOCD` also performs other important work as part of its translation for the debugging of the ARM Cortex-M based microcontroller on your STM32F3DISCOVERY development board:

    It knows how to interact with the memory mapped registers used by the ARM CoreSight debug peripheral. It is these CoreSight registers that allow for:
    Breakpoint/Watchpoint manipulation
    Reading and writing of the CPU registers
    Detecting when the CPU has been halted for a debug event
    Continuing CPU execution after a debug event has been encountered
    etc.
    It also knows how to erase and write to the microcontroller's FLASH

    ```bash
    brew install armmbed/formulae/arm-none-eabi-gcc
    brew install openocd
    ```

    After that, you will find the `OpenOCD` interface configuration and target configuration at here:

    _`/usr/local/Cellar/open-ocd/{VERSION_NUMBER}/share/openocd/scripts/interface/stlink-v2-1.cfg`_
    _`/usr/local/Cellar/open-ocd/{VERSION_NUMBER}/share/openocd/scripts/target/stm32f4x.cfg`_

- **ST-LINK** to use burn your program into `STM32F4` hardware

    ```bash
    brew install stlink
    ```


[More detail here](https://rust-embedded.github.io/book/intro/tooling.html)

</br>

#### <a name="create-demo-project-from-template">2.2 Create `demo` project from template</a>

```bash
cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
🤷  Project Name: demo
```

</br>

#### <a name="run-hello-example-in-qemu">2.3 Run hello example in `QEMU` and set break point in `ARM GDB`</a>

- Add the below settings to `.cargo/config` 

    ```js
    [target.thumbv7em-none-eabi]
    # Settings below will make `cargo run` execute programs on QEMU
    # Normal version
    # runner = "qemu-system-gnuarmeclipse -cpu cortex-m4 -mcu STM32F407VG -machine STM32F4-Discovery -nographic -semihosting-config enable=on,target=native -kernel" # Normal version (show dev board UI)
    # Normal version (with dev board UI)
    # runner = "qemu-system-gnuarmeclipse -cpu cortex-m4 -mcu STM32F407VG -machine STM32F4-Discovery -semihosting-config enable=on,target=native -kernel"
    
    # Debug version, QEMU wll wait for the GDB connection from TCP port 3333 and halt the machine.
    # runner = "qemu-system-gnuarmeclipse -gdb tcp::3333 -S -cpu cortex-m4 -mcu STM32F407VG -machine STM32F4-Discovery -nographic -semihosting-config enable=on,target=native -kernel"
    # Debug version (with dev board UI), QEMU wll wait for the GDB connection from TCP port 3333 and halt the machine.
    runner = "qemu-system-gnuarmeclipse -gdb tcp::3333 -S -cpu cortex-m4 -mcu STM32F407VG -machine STM32F4-Discovery -semihosting-config enable=on,target=native -kernel"


    [build]
    # As we use STM32F4DISCOVERY, then pick the `Cortex-M4F`
    # target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
    
    # But if running in QEMU, then we can't use the FPU version, as QEMU doesn't support!!!
    target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
    ```

- Run `qemu` in debug mode with `cargo run --example hello`

- Now we're able to run arm version `GDB` and connect to debug TCP port:

    ```js
    arm-none-eabi-gdb \
    -q target/thumbv7em-none-eabi/debug/examples/hello \
    -ex "target remote:3333"

    # List the `main` fn then we can set breakpoint to the line number we want:
    (gdb) list main
    (gdb) break 13
    Breakpoint 1 at 0x484: file examples/hello.rs, line 13.

    (gdb) step
    (gdb) print &x
    (gdb) next
    (gdb) continue 
    (gdb) stop 
    (gdb) quit
    ```

</br>
    
#### <a name="run-hello-example-in-hardware">2.3.1 Run hello example in hardware<a>

- First, make sure you go through this [step](https://rust-embedded.github.io/book/intro/install/verify.html) to 
make sure all hardware connections already work.

- We use `--target` in `cargo` command, then we don't need to modify `.cargo/config` again

    ```bash
    cargo run --target thumbv7em-none-eabihf --example hello
    ```

- For debugging, plz follow steps [here](https://rust-embedded.github.io/discovery/05-led-roulette/flash-it.html)

</br>

#### <a name="debugging-in-vim-gdb">2.3.2 Debugging in `vim` with the `ARM GDB`</a>

- Todo: [Tutorial Link Here](https://www.dannyadam.com/blog/2019/05/debugging-in-vim/)


</br>

## <a name="how-to-choose-which-crate-to-start">3. How to choose which crate (rust's library) to start</a>

![How to choose which crate to start](book/images/how-to-choice-which-crate.png)

In `Rust` world, you got several options to start your project, it totally depends on your situation, below are the tips:

- **Micro-architecture Crate**

   This sort of crate handles any useful routines common to the processor core your microcontroller is using, as well as any peripherals that are common to all micro-controllers that use that particular type of processor core. For example the `cortex-m` crate gives you functions to enable and disable interrupts, which are the same for all `Cortex-M` based micro-controllers. It also gives you access to the 'SysTick' peripheral included with all `Cortex-M` based micro-controllers.

- **Peripheral Access Crate (PAC)**

   This sort of crate is a thin wrapper over the various memory-wrapper registers defined for your particular part-number of micro-controller you are using. For example, [`stm32f4`](https://crates.io/crates/stm32f4) for the **ST-Micro STM32F40x series**. Here, you'll be interacting with the registers directly, following each peripheral's operating instructions given in your micro-controller's Technical Reference Manual.

- **HAL (Hardware Abstraction Layer) Crate**

    These crates offer a more user-friendly API for your particular processor, often by implementing some common traits defined in [`embedded-hal`](https://crates.io/crates/embedded-hal). For example, [`stm32f4xx-hal`](https://crates.io/crates/stm32f4xx-hal) crate might offer a Serial struct, with a constructor that takes an appropriate set of GPIO pins and a baud rate, and offers some sort of write_byte function for sending data. See the chapter on Portability for more information on embedded-hal.

- **Board Crate**

    These crates go one step further than a HAL Crate by pre-configuring various peripherals and GPIO pins to suit the specific developer kit or board you are using, such as [`stm32f3-discovery`](https://crates.io/crates/stm32f3-discovery) for the **STM32F3DISCOVERY** board.


Let's try that one-by-one:

### <a name="the-basic-example">3.1 The basic example</a>

- Make sure your `Cargo.toml` has the settings below:

    ```rust
    [dependencies]
    cortex-m = "0.6.0"
    cortex-m-rt = "0.6.10"
    cortex-m-semihosting = "0.3.3"
    # panic-halt = "0.2.0"
    
    # For debugging purpose, enable `exit` feature
    panic-semihosting = { version = "0.5.3", features = ['exit'] }
    ```

- Make sure your `.cargo/config` has the settings below:

    ```rust
    [target.thumbv7em-none-eabi]
    # Settings below will make `cargo run` execute programs on QEMU
    # Normal version
    runner = "qemu-system-gnuarmeclipse -cpu cortex-m4 -mcu STM32F407VG -machine STM32F4-Discovery -nographic -semihosting-config enable=on,target=native -kernel"
    ```


- Create [`demo/examples/basic.rs`](demo/examples/basic.rs) with the following code:

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
    // hardware!!! Better to use the config to enable the call or not.
    use cortex_m_semihosting::{dbg, hprintln};
    
    // `panic_semihosting` will call `debug::EXIT_FAILURE` after logging the
    // panic message.
    use panic_semihosting as _;
    
    // Set to `false` when u don't need that anymore
    const ENABLE_DEBUG: bool = true;
    
    // We will use the `entry` attribute from the `cortex_m_rt` crate to define
    // the entry point. The entry point function must have signature `fn() -> !;`
    // which means can't return, as the program never terminates.
    #[entry]
    fn main() -> ! {
        // `hprintln` returns `Result<(),()>`
        if ENABLE_DEBUG {
            let _ = hprintln!("Basic STM32F4 demo is running >>>>>");
        }
    
        // Get the singleton `Peripherals` instance. This method can only
        // successfully called **once()**, that's why return an `Option`.
        let peripherals = Peripherals::take().unwrap();
    
        // You can't do this, as `cortex_m::Peripherals` cannot be formatted
        // using `{:?}` because it doesn't implement `core::fmt::Debug`.
        // dbg!(peripherals);
    
        let x = 10;
        if ENABLE_DEBUG {
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



- How to run it with hot-load behavior

    ```rust
    # cd demo
    cargo watch -c --exec 'run --example basic'
    ```
    Every time you save `demo/examples/basic.rs`, `cargo run --example basic` will run again.

    ```rust
    (qemu) System timer demo is running >>>>>
    x is 10
    [examples/basic.rs:35] x = 10
    panicked at 'assertion failed: `(left == right)`
    left: `10`,
    right: `8`', examples/basic.rs:36:5
    [Finished running. Exit status: 1]
    ```
</br>


### <a name="the-gpio-led-example">3.2 The GPIO LED blinking example</a>

In this example, we will use `HAL` crate rather than using `PAC` which provides a better high-level abstraction API for coding.
The `HAL` crate is [`stm32f4xx-hal`](https://crates.io/crates/stm32f4xx-hal), also make sure enable the `stm32f407` feature, as
that's the `MCU` we use.

- Make sure your `Cargo.toml` has the settings below:

    ```rust
    [dependencies]
    cortex-m = "0.6.0"
    cortex-m-rt = "0.6.10"
    cortex-m-semihosting = "0.3.3"
    # panic-halt = "0.2.0"
    
    # For debugging purpose, enable `exit` feature
    panic-semihosting = { version = "0.5.3", features = ['exit'] }
    
    # PAC (Peripheral Access Crate)
    # stm32f4 = { version = "0.11.0", features = ["stm32f407", "rt"] }
    
    # HAL (Hardware Abstraction Layer)
    stm32f4xx-hal = { version = "0.8.3", features = ['stm32f407'] }
    ```

- Make sure your `.cargo/config` has the settings below:

    We need to use the `runner` config without `-nographic`, then `QEMU` will open a dev board GUI, so we can see the blinking LED.

    ```rust
    [target.thumbv7em-none-eabi]
    # Settings below will make `cargo run` execute programs on QEMU
    # Normal version (with dev board UI)
    runner = "qemu-system-gnuarmeclipse -cpu cortex-m4 -mcu STM32F407VG -machine STM32F4-Discovery -semihosting-config enable=on,target=native -kernel"
    ```


- Create [`demo/examples/gpio_led.rs`](demo/examples/gpio_led.rs) with the following code:

    ```rust
    #![no_std]
    #![no_main]
    
    use cortex_m_rt::entry;
    use cortex_m_semihosting::hprintln;
    use panic_semihosting as _;
    
    use crate::hal::{prelude::*, stm32};
    
    // This is very important!!!
    use stm32f4xx_hal as hal;
    
    // Import from `stm32f4xx_hal`
    use hal::{
        delay::Delay,
        gpio::{
            gpiod::{Parts, PD12, PD13, PD14, PD15},
            Output, PushPull,
        },
        rcc::Rcc, // Constrained RCC peripheral
    };
    
    // Set to `false` when u don't need that anymore
    const ENABLE_DEBUG: bool = true;
    
    #[entry]
    fn main() -> ! {
        if ENABLE_DEBUG {
            let _ = hprintln!("STM32F4 GPIO Led demo is running >>>>>");
        }
    
        let stm32407_peripherals = stm32::Peripherals::take().unwrap();
        let cortex_m_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();
    
        // Set up the LEDs. Below LED info copied from STM32F4Discovery user manual:
        //
        // • User LD3: orange LED is a user LED connected to the I/O PD13 of the STM32F407VGT6.
        // • User LD4: green LED is a user LED connected to the I/O PD12 of the STM32F407VGT6.
        // • User LD5: red LED is a user LED connected to the I/O PD14 of the STM32F407VGT6.
        // • User LD6: blue LED is a user LED connected to the I/O PD15 of the STM32F407VGT6.
        //
        // All those LED pins group into the port `D` which defined here:
        // https://docs.rs/stm32f4xx-hal/0.8.3/stm32f4xx_hal/stm32/struct.GPIOD.html#method.split
        //
        // `GPIOD.splt()` return a `Parts` struct instance which include all pins with the default
        // <MODE<type state>> which is `Input<Floating>`:
        // https://docs.rs/stm32f4xx-hal/0.8.3/stm32f4xx_hal/gpio/gpiod/struct.Parts.html
        let gpiod: Parts = stm32407_peripherals.GPIOD.split();
    
        // Take all those LED pins and convert into `Output` mode with `PushPull` type state
        let mut green_led: PD12<Output<PushPull>> = gpiod.pd12.into_push_pull_output();
        let mut orange_led: PD13<Output<PushPull>> = gpiod.pd13.into_push_pull_output();
        let mut red_led: PD14<Output<PushPull>> = gpiod.pd14.into_push_pull_output();
        let mut blue_led: PD15<Output<PushPull>> = gpiod.pd15.into_push_pull_output();
    
        // Set up the system clock. We want to run at 16Mhz for this one.
        let constrained_rcc_peripheral: Rcc = stm32407_peripherals.RCC.constrain();
        let clocks = constrained_rcc_peripheral.cfgr.sysclk(16.mhz()).freeze();
    
        // Create a delay abstraction based on SysTick
        let mut delay = Delay::new(cortex_m_peripherals.SYST, clocks);
    
        // I don't know how the `sysclk` works and how to set the correct `Mhz`, but for now,
        // the `excepted_delay_time_in_ms` needs to cut half for getting the correct delay time.
        let excepted_delay_time_in_ms = 1000u32;
        let delay_time_in_ms = (excepted_delay_time_in_ms / 2) as u32;
    
        loop {
            // On for 1s
            green_led.set_high().unwrap();
            orange_led.set_high().unwrap();
            red_led.set_high().unwrap();
            blue_led.set_high().unwrap();
    
            delay.delay_ms(delay_time_in_ms);
    
            // off for 1s
            green_led.set_low().unwrap();
            orange_led.set_low().unwrap();
            red_led.set_low().unwrap();
            blue_led.set_low().unwrap();
    
            delay.delay_ms(delay_time_in_ms);
        }
    }

    ```

- How to run it with hot-load behavior

    ```rust
    # cd demo
    cargo watch -c --exec 'run --example gpio_led'
    ```
    Every time you save `demo/examples/gpio_led.rs`, `cargo run --example gpio_led` will run again.

    And you should be able to see the console log like below:

    ![gpio-led-demo.png](book/images/gpio-led-demo.png)

    Also, see the blinking LED in dev board UI:

    ![gpio-led-demo.png](book/images/gpio-led-demo.gif)

</br>
