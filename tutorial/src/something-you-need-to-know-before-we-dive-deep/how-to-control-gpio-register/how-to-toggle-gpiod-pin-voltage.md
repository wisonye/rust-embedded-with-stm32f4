#### <a name="how-to-toggle-gpiod-pin-voltage">5.3.4 How to set the `GPIO` port `D` (pin12 ~ pin15) to `High` or `Low`</a>

Let's go to the [`reference manual`](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/stm32f4-reference-manual.pdf) page 284, then we will see the info below:

<hr>

![gpio-bssr.png](../../../images/gpio-bssr.png)

<hr>

So what information we got here?

- **GPIOD_BSSR** uses to set the register value (`High` or `Low`)

    - How to set `High`, take a look at the green highlighted part:
        - `bit0` to `bit15` means **pin 0** to **pin 15** and use to set `High`
        - After setting to `1`, it will set the corresponding **GPIOD_ODR** register bit
        - Nothing will happen when setting the `0` to the specified bit
        - For setting the **pin 12** to `High`, we need to set `bit12` to `1`
        - ...
        - For setting the **pin 15** to `High`, we need to set `bit15` to `1`

        </br>

    - How to set `Low`, take a look at the red highlighted part:
        - `bit16` to `bit31` means **pin 0** to **pin 15** and use to set `Low`
        - After setting to `1`, it will set the corresponding **GPIOD_ODR** register bit
        - Nothing will happen when setting the `0` to the specified bit
        - For setting the **pin 12** to `Low`, we need to set `bit(12 + 16)` to `1`
        - ...
        - For setting the **pin 15** to `Low`, we need to set `bit(15 + 16)` to `1`


    </br>

- Then we're able to write `rust` code to set **pin 12~15** to `High` or `Low` like below:

    ```rust
    // GPIO port bit set/reset register (GPIOx_BSRR) address, `reference manual` page 284.
    const GPIOD_BSRR: u32 = GPIOD_REGISTER + 0x18;
    let gpiod_bsrr_mut_ptr = GPIOD_BSRR as *mut u32;

    // Setup GPIOD.P12 ~ P15 to output mode with `

    unsafe {
        // Set bit (pin) 12 ~ 15 to `1` to turn on 4 LEDs. As the "GPIOD_BSRR" does nothing when
        //
        // As the "GPIOD_BSRR" does nothing when setting bit to `0`, so actually, we even don't 
        // need the `|=` for keeping the previous value.
        *gpiod_bsrr_mut_ptr = (1 << 12) | (1 << 13) | (1 << 14) | (1 << 15);
    }
    
    #[cfg(feature = "enable-debug")]
    let _ = hprintln!("\nDelay 1s......\n");

    delay.delay_ms(delay_time_in_ms);

    unsafe {
        // Set bit (pint) 12 + 16, 13 + 16 to `1` to turn off 2 LEDs. As the "GPIOD_BSRR" does nothing when
        //
        // As the "GPIOD_BSRR" does nothing when setting bit to `0`, so actually, we even don't 
        // need the `|=` for keeping the previous value.
        *gpiod_bsrr_mut_ptr = (1 << (12 + 16)) | (1 << (13 + 16));
    }
    ```
