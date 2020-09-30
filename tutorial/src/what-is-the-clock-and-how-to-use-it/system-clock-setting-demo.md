### <a name="system-clock-demo">6.7 Finally, let's put all together: System clock demo</a>

- Make sure your `.cargo/config` has the settings below:

    We need to use the `runner` config without `-nographic`, then `QEMU` will open a dev board GUI, so we can see the blinking LED.

    ```rust
    [target.thumbv7em-none-eabi]
    # Settings below will make `cargo run` execute programs on QEMU
    # Normal version (with dev board UI)
    runner = "qemu-system-gnuarmeclipse -cpu cortex-m4 -mcu STM32F407VG -machine STM32F4-Discovery -semihosting-config enable=on,target=native -kernel"
    ```

- All the previous source code links:

    - [demo/src/clock_frequency.rs](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/clock_frequency.rs)
    - [demo/src/rcc_clock_settings.rs](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/rcc_clock_settings.rs)
    - [demo/src/clock_utils.rs](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/clock_utils.rs)
    - [demo/src/register_utils/flash_access_control_register.rs](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/register_utils/flash_access_control_register.rs)
    - [demo/src/register_utils/rcc_clock_config_register.rs](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/register_utils/rcc_clock_config_register.rs)
    - [demo/src/register_utils/rcc_clock_control_register.rs](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/register_utils/rcc_clock_control_register.rs)
    - [demo/src/register_utils/rcc_pll_config_register.rs](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/register_utils/rcc_pll_config_register.rs)

- Create [`demo/src/bin/setup_and_print_system_clock.rs`](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/bin/setup_and_print_system_clock.rs) with the following code:

    ```rust
    #![allow(warnings)]
    #![no_std]
    #![no_main]
    
    // We have to use the attribute to locate the module file!
    #[path = "../clock_frequency.rs"]
    mod clock_frequency;
    #[path = "../clock_utils.rs"]
    mod clock_utils;
    #[path = "../register_utils/flash_access_control_register.rs"]
    mod flash_access_control_register;
    #[path = "../register_utils/rcc_clock_config_register.rs"]
    mod rcc_clock_config_register;
    #[path = "../register_utils/rcc_clock_control_register.rs"]
    mod rcc_clock_control_register;
    #[path = "../rcc_clock_settings.rs"]
    mod rcc_clock_settings;
    #[path = "../register_utils/rcc_pll_config_register.rs"]
    mod rcc_pll_config_register;
    
    use cortex_m_rt::entry;
    use panic_semihosting as _;
    
    #[cfg(feature = "enable-debug")]
    use cortex_m_semihosting::hprintln;
    
    use crate::clock_utils::{ClockSource, RccClocks};
    
    ///
    #[entry]
    fn main() -> ! {
        #[cfg(feature = "enable-debug")]
        let _ = hprintln!("STM32F4 setup and print system clock demo is running >>>>>");
    
        #[cfg(feature = "enable-debug")]
        {
            // RccClocks::setup_system_clock(ClockSource::Hsi);
            // RccClocks::setup_system_clock(ClockSource::HsiThroughPll);
            RccClocks::setup_system_clock(ClockSource::HseThroughPll);

            // Print all related registers debug info
            RccClocks::print_system_clock_info();
        }
    
        loop {}
    }
    ```

- About different feature settings:

    - In `Cargo.toml`, we defined 2 optional features:

        ```rust
        [features]
        use-weact-black-pill = []
        use-stm32f407g-disc1 = []
        ```


    - In [demo/src/rcc_clock_settings.rs](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/rcc_clock_settings.rs), 
    we defined different conditional compilation block against to different feature:

        ```rust
        #[cfg(feature = "use-stm32f407g-disc1")]
        pub mod clock_source_selecting {
            // constants define here
        }

        #[cfg(feature = "use-weact-black-pill")]
        pub mod clock_source_selecting {
            // constants define here
        }
        ```

- How to run

    - Use `use-stm32f407g-disc1` feature:
        ```rust
        // cd demo
        cargo watch -c --exec 'run --bin setup_and_print_system_clock --features "enable-debug use-stm32f407g-disc1"'
        ```

        Then you should be able to see the console log like below:

        ```
        QEMU 2.8.0-9 monitor - type 'help' for more information
        (qemu) STM32F4 setup and print system clock demo is running >>>>>
        
        [ RccClocks ]: {
            hsi: "16MHz",
            hse: "8MHz",
            clock_source: HseThroughPll,
            system_clock: "168MHz",
            hardware_cpu_clock: "168MHz",
            pll_m: Some(
                8,
            ),
            pll_n: Some(
                336,
            ),
            pll_p: Some(
                2,
            ),
            pll_q: Some(
                4,
            ),
            ahb_prescaler: Some(
                1,
            ),
            apb1_peripheral_clock: "42MHz",
            apb1_timer_clock: "84MHz",
            apb2_peripheral_clock: "84MHz",
            apb2_timer_clock: "168MHz",
        }
        clock_switch_status_bits: 0b10
        still_not_stable: false
        Waiting for clock switch become stable>>>>>
        
        [ RCC clock control register (RCC_CR) ]:
        value: 0000000011000000000000000000000000
        High speed internal (HSI) clock enable: false
        High speed internal (HSI) clock stable: false
        High speed external (HSE) clock enable: false
        High speed external (HSE) clock stable: false
        HSE oscillator bypassed with an external clock: false
        Clock security system enable: false
        Main PLL (PLL) enable: true
        Main PLL (PLL) is ready: true
        PLLI2S is enable: false
        PLLI2S is ready: false
        
        [ RCC clock configuration register (RCC_CFGR) ]:
        value: 0000000000000000000000000000001010
        System clock selected: PllSelectedAsSytemClock   // bits: 0b10
        System clock status: PllUsedAsSytemClock         // bits: 0b10
        AHB prescaler: 1                                 // bits: 0b0000
        APB low speed prescaler (APB1): 1                // bits: 0b000
        APB high speed prescaler (APB2): 1               // bits: 0b000
        
        [ RCC PLL configuration register (RCC_PLLCFGR) ]:
        RCC_PLLCFGR value: 0b00000100010000100101010000001000
        Main PLL M: Ok(8),      // bits: 0b001000
        Main PLL N: Ok(336),    // bits: 0b101010000
        Main PLL P: Ok(2),      // bits: 0b10
        Main PLL q: Ok(4),      // bits: 0b0100
        Main PLL source: "HSE", // bits: 0b1
        
        [ Flash access control register (FLASH_ACR) ]:
        FLASH_ACR value: 0b00000000000000000000011100000101
        Falsh read latency: FiveWaitState6CpuCycles,    // bits: 0b101
        Prefetch enabled: true,                         // bits: 0b1
        Instruction cache enabled: true,                // bits: 0b1
        Data cache enabled: true,                       // bits: 0b1bash
        ```

        </br>

    - Use `use-weact-black-pill` feature:

        ```rust
        // cd demo
        cargo watch -c --exec 'run --bin setup_and_print_system_clock --features "enable-debug use-weact-black-pill"'
        ```

        And you should be able to see the console log like below:


        ```bash
        QEMU 2.8.0-9 monitor - type 'help' for more information
        (qemu) STM32F4 setup and print system clock demo is running >>>>>

        [ RccClocks ]: {
            hsi: "16MHz",
            hse: "25MHz",
            clock_source: HseThroughPll,
            system_clock: "100MHz",
            hardware_cpu_clock: "100MHz",
            pll_m: Som (
                25,
            ),
            pll_n: Som (
                200,
            ),
            pll_p: Som (
                2,
            ),
            pll_q: Som (
                4,
            ),
            ahb_prescaler: Som (
                1,
            ),
            apb1_peripheral_clock: "50MHz",
            apb1_timer_clock: "100MHz",
            apb2_peripheral_clock: "100MHz",
            apb2_timer_clock: "100MHz",
        }
        clock_switch_status_bits: 0b10
        still_not_stable: false
        Waiting for clock switch become stable>>>>>

        [ RCC clock control register (RCC_CR) ]:
        value: 0000000011000000000000000000000000
        High speed internal (HSI) clock enable: false
        High speed internal (HSI) clock stable: false
        High speed external (HSE) clock enable: false
        High speed external (HSE) clock stable: false
        HSE oscillator bypassed with an external clock: false
        Clock security system enable: false
        Main PLL (PLL) enable: true
        Main PLL (PLL) is ready: true
        PLLI2S is enable: false
        PLLI2S is ready: false

        [ RCC clock configuration register (RCC_CFGR) ]:
        value: 0000000000000000000000000000001010
        System clock selected: PllSelectedAsSytemClock   // bits: 0b10
        System clock status: PllUsedAsSytemClock         // bits: 0b10
        AHB prescaler: 1                                 // bits: 0b0000
        APB low speed prescaler (APB1): 1                // bits: 0b000
        APB high speed prescaler (APB2): 1               // bits: 0b000

        [ RCC PLL configuration register (RCC_PLLCFGR) ]:
        RCC_PLLCFGR value: 0b00000100010000100011001000011001
        Main PLL M: Ok(25),     // bits: 0b011001
        Main PLL N: Ok(200),    // bits: 0b011001000
        Main PLL P: Ok(2),      // bits: 0b10
        Main PLL q: Ok(4),      // bits: 0b0100
        Main PLL source: "HSE", // bits: 0b1

        [ Flash access control register (FLASH_ACR) ]:
        FLASH_ACR value: 0b00000000000000000000011100000011
        Falsh read latency: ThreeWaitState4CpuCycles,   // bits: 0b011
        Prefetch enabled: true,                         // bits: 0b1
        Instruction cache enabled: true,                // bits: 0b1
        Data cache enabled: true,                       // bits: 0b1
        ```
