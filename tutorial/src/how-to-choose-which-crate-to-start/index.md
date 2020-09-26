## <a name="how-to-choose-which-crate-to-start">4. How to choose which crate (rust's library) to start</a>

![How to choose which crate to start](../images/how-to-choice-which-crate.png)

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
