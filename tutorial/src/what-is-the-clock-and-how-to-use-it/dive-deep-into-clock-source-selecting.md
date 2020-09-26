### <a name="dive-deep-into-clock-source-selecting">6.5 Let's dive deep into the `clock source selecting`</a>

First thing first, there are a few kinds of `Clocks` in the [`reference manual`](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/stm32f4-reference-manual.pdf) starts from page **215**, but we only need to focus on the three of them below:

- **HSE**

    High-Speed External clock signal, usually, it's external onboard oscillator hardware with the fixed working frequency. Make sure you check the hardware datasheet before setting `HSE` value (unit in Hertz).

- **HSI**
    
    High-Speed Internal clock signal, it's an internal 16 MHz RC oscillator.

- **PLL**

    This stuff is complicated, as it stands for **Phase-Locked Loop**. It sounds scary for you, as you have no idea what that means. Briefly, it takes the input frequency and uses different factor settings to change the output frequency.


So, let's have a close look at the `clock source selecting`:

We can pick one of the path below to as the clock source and
calculate the `SYSCLK` working frequency:

- **HSI** ------------------------------------------------------> **SYSCLK**
- **HSE** -----------------------------------------------------> **SYSCLK**
- **PLL** ----> (HSI or HSE) / PLL_M * PLL_N / PLL_P ---> **SYSCLK**

As you can see, it's not that hard:) 

When the expected working frequency is higher than `HSI` or `HSE` can offer, then you have to use `PLL` as clock source!!!

Let's have a look at them one by one:

- **HSI --> SYSCLK**:

    ![hsi-as-clock-source.png](../../images/hsi-as-clock-source.png)

    This is the case of **HSI** as the clock source. You can see the **SYSCLK** get clock signal directly from **HSI** and the final working frequency is **16MHz**.

- **HSE --> SYSCLK**:

    ![hse-bypass-as-clock-source.png](../../images/hse-bypass-as-clock-source.png)

    This is the case of **HSE** as the clock source, also call `HSE bypass mode` (as not go through **PLL**). You can see the **SYSCLK** get clock signal directly from `HSE` and the final working frequency is **25MHz** (equal to **HSE** offered frequency).

- **PLL** ----> (HSI or HSE) / PLL_M * PLL_N / PLL_P ---> **SYSCLK**:

    ![hse-pll-as-clock-source.png](../../images/hse-pll-as-clock-source.png)

    This is the case of **HSE through PLL** as the clock source. You can see the **SYSCLK** get clock signal from `HSE` first, and then go through **PLL** factors (**M/N/P**) calculation, finally, got the working frequency is **168MHz**.

    For the **PLL_M/N/P**, usually, you don't need to worry about, as `STMCubeMX` will figure out the combination for you when you change the expected frequency in **HCLK**. But you can change it by youself.

    For example, when I change the **HCLK** frequency to **100MHz** like below, `STMCubeMX` calculates the combination for me:

    ![pll-combination-2.png](../../images/pll-combination-2.png)

    But I can change it like below and got the same result:

    ![pll-combination-1.png](../../images/pll-combination-1.png)

    Here are the rules if you want to change that combination by youself:

    - When using **PLL** as clock source, we should calculate **SYSCLK** working frequency with the following formula:

        ```
        PLL_VCO = (HSE_FREQUENCY or HSI_FREQUENCY / PLL_M) * PLL_N
        SYSCLK = PLL_VCO / PLL_P

        `VCO` stands for `Voltage-Controlled Oscillator`
        ```

    - Tips for picking the right factor value:

        - `PLL_M`: We can always make it equal to the `HSI_FREQUENCY`
         or `HSE_FREQUENCY`, then:

            `(HSE_FREQUENCY or HSI_FREQUENCY / PLL_M)` always return `1`

            which more easy to to do the rest calculating. 
            
        - `PLL_P`: We can try start from `2`, then `PLL_M` and `PLL_P`
          already fixed, only left the `PLL_N` to choose.

        But keep in mind that `PLL_M/N/P` have to in the allowed range provided in the reference manual (page 227). 

        So the suggestion is always going to `STM32CubeMX` and use the automatic combination or change your settings in the UI, as it will tell you when your value is not working.
