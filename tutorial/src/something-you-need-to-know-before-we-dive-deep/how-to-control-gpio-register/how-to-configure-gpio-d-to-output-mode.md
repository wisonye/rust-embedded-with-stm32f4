#### <a name="how-to-configure-gpio-d-to-output-mode">5.3.3 How to configure the `GPIO` port `D` to **output** mode</a>

Let's go to the [`reference manual`](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/stm32f4-reference-manual.pdf) page 281, then we will see the info below:

<hr>

![gpio-moder.png](../../../images/gpio-moder.png)

<hr>

So what information we got here?

- **GPIOD_MODER** uses to configure the register mode, take a look at the blue highlighted part:
    - `MODER12` means **pin 12**, `MODER13` means **pin 12**, ..., `MODER15` means **pin 15**
    - Each pin use 2 bits to set the register to the particular mode
    - For setting the **pin 12** to output mode, we need to set `01` to `bit25` and `bit24`
    - ...
    - For setting the **pin 15** to output mode, we need to set `01` to `bit31` and `bit30`

    </br>

- **GPIOD_OTYPER** uses to configure the register output type, take a look at the orange highlighted part:
    - Each pin use 1 bit to set the register to the particular output mode
    - For setting the **pin 12** to `push-pull` mode, we need to set `0` to `bit12`
    - ...
    - For setting the **pin 15** to `push-pull` mode, we need to set `0` to `bit15`

    </br>

- Then we're able to write `rust` code to configure **GPIOD** work in **output** mode like below:

    ```rust
    // `GPIOD` register mapping address, in `reference manual` page 65, (`STM32F4xx register boundary addresses`).
    const GPIOD_REGISTER: u32 = 0x40020c00;

    // GPIO port mode register (GPIOx_MODER) address, `reference manual` page 281.
    const GPIOD_MODER: u32 = GPIOD_REGISTER + 0x00;
    let gpiod_moder_mut_ptr: *mut u32 = GPIOD_MODER as *mut u32; // Mutable raw pointer
    let gpiod_moder_ptr: *const u32 = GPIOD_MODER as *const u32; // Immutable raw pointer
    unsafe {
        // Set `GPIOD` pin12 ~ pin15 to OUTPUT mode

        // Keep the prev value and add (`|`) new setting:
        // bit 25, 24 set to `01`
        // bit 27, 26 set to `01`
        // bit 29, 28 set to `01`
        // bit 31, 30 set to `01`
        *gpiod_moder_mut_ptr |= (1 << 24) | (1 << 26) | (1 << 28) | (1 << 30);

        // Let's print the "GPIOD_MODER" register bit value (32bit, 4 bytes), and it should be:
        // 0b01010101000000000000000000000000
        // 
        // From right to left is bit0 ~ bit31, only bit24, bit26, bit 28, bit30 set to `1`.
        #[cfg(feature = "enable-debug")]
        let _ = hprintln!("GPIOD_MODER: {:#034b}", *gpiod_moder_ptr);
    }

    // GPIO port output type register (GPIOx_OTYPER) address, `reference manual` page 281.
    // As the output type `push-pull` is `0`, then we don't need to set `GPIOD_OTYPER` explicitly.
    // const GPIOD_OTYPER: u32 = GPIOD_REGISTER + 0x04;
    ```
