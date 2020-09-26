### <a name="how-to-control-gpio-register">5.3 How to control the GPIO register</a>
Usually, we need a few steps to control the particular peripheral registers:
- Enable it
- Configure it
- Read data from or write data to it

All steps above need the register memory address and we can find it in the [`reference manual`](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/stm32f4-reference-manual.pdf).

In the next demo, we want to turn-on and turn-off the onboard LEDs. For the first step, we need to open 
[Discovery_kit_withlSTM32F407VG_MCU_user_manual.pdf](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/Discovery_kit_withlSTM32F407VG_MCU_user_manual.pdf) and jump to page 16, then we can see the info below:

```
• User LD3: orange LED is a user LED connected to the I/O PD13 of the
STM32F407VGT6.
• User LD4: green LED is a user LED connected to the I/O PD12 of the STM32F407VGT6.
• User LD5: red LED is a user LED connected to the I/O PD14 of the STM32F407VGT6.
• User LD6: blue LED is a user LED connected to the I/O PD15 of the STM32F407VGT6.
```

So we know that all the LEDs are connected to the **GPIO** port **D** with the `pin 12 ~ pin 15`.

Let's walk through all those steps to reach our goal:
