### <a name="dive-deep-into-clock-source-selecting">6.6 Let's talk about `clock source selecting` registers handling</a>

We need to go through all the steps below for picking a clock source target to the particular working frequency:


1. Enable **`HSE`** and wait for it stable if we use **`HSE`**.

2. Set the `AHB prescaler, APB1 prescaler, APB2 prescaler`.

3. Setup flash read latency.

4. Set **`PLL`** factors `MNPQ`.

5. Enable **`PLL`** and wait for it stable.

For each step, we need to deal with different register, we will walk through them one by one later.

Before that, let's define some `enum` types and `struct` to describe the `clock`:

- The clock source type we will use in the demo:

    ```rust
    #[derive(Debug, Clone, PartialEq)]
    pub enum ClockSource {
        Hsi,
        HsiThroughPll,
        HseThroughPll,
    }
    ```

- The clock data structure which includes all related information in the `clock tree diagram`, and we only focus on related settings and the final working frequency output:

    ```rust
    pub struct RccClocks {
        // HSI fixed frequency
        hsi: Option<MegaHertz>,
        // Onboard HSE fixed frequency
        hse: Option<MegaHertz>,
        // The clock source we picked
        clock_source: ClockSource,
        // SYSCLK calculated frequency
        system_clock: Option<MegaHertz>,
        // HCLK calculated frequency
        hardware_cpu_clock: Option<MegaHertz>,
        // AHB bus prescaler
        ahb_prescaler: Option<u32>,
        // PLL factors
        pll_m: Option<u32>,
        pll_n: Option<u32>,
        pll_p: Option<u32>,
        pll_q: Option<u32>,
        // APB1 periheral calculated frequency
        apb1_peripheral_clock: Option<MegaHertz>,
        // APB1 timer calculated frequency
        apb1_timer_clock: Option<MegaHertz>,
        // APB2 periheral calculated frequency
        apb2_peripheral_clock: Option<MegaHertz>,
        // APB2 timer calculated frequency
        apb2_timer_clock: Option<MegaHertz>,
    }
    ```

    As you can see, the `MegaHertz` frequency unit type above allows easier to convert a number (`u32`) into frequency unit (`Hz, KHz, MHz`), here is the source code:

    ```rust
    #[derive(PartialEq, PartialOrd, Clone, Copy)]
    pub struct Hertz(pub u32);

    #[derive(PartialEq, PartialOrd, Clone, Copy)]
    pub struct KiloHertz(pub u32);

    #[derive(PartialEq, PartialOrd, Clone, Copy)]
    pub struct MegaHertz(pub u32);

    // impl From<MegaHertz> for u32 {
    // fn from(value: MegaHertz) -> u32 {
    // value.0
    // }
    // }

    impl From<u32> for MegaHertz {
        fn from(value: u32) -> MegaHertz {
            MegaHertz(value / 1_000_000)
        }
    }

    impl From<u32> for KiloHertz {
        fn from(value: u32) -> KiloHertz {
            KiloHertz(value / 1_000)
        }
    }

    impl From<u32> for Hertz {
        fn from(value: u32) -> Hertz {
            Hertz(value)
        }
    }

    impl Into<Hertz> for KiloHertz {
        fn into(self) -> Hertz {
            Hertz(self.0 * 1_000)
        }
    }

    impl Into<Hertz> for MegaHertz {
        fn into(self) -> Hertz {
            Hertz(self.0 * 1_000_000)
        }
    }

    impl Into<KiloHertz> for MegaHertz {
        fn into(self) -> KiloHertz {
            KiloHertz(self.0 * 1_000)
        }
    }
    ```

So, let's have a look at each register one by one:)
