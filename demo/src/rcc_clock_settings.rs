// ------ RCC Clock Frequency related -------------------------
//
// For the `HSI` (High-Speed Internal clock signal), it's an
// internal 16 MHz RC oscillator.
//
// For the `HSE` (High-Speed External clock signal), usually,
// it's external onboard oscillator hardware with the fixed
// working frequency. Make sure you check the hardware datasheet
// before setting `HSE` value (unit in Hertz).
//
// We can pick one of the path below to as the clock source and
// calculate the `SYSCLK` working frequency:
//
// HSI ----------------------------------------------> SYSCLK
// HSE ----------------------------------------------> SYSCLK
// PLL ----> (HSI or HSE) / PLL_M * PLL_N / PLL_P ---> SYSCLK
//
// When the expected working frequency is higher than `HSI` or
// `HSE` can offer, then you have to use `PLL` as clock source!!!
//
// When using `PLL` as clock source, we need to set the correct
// PLL factors (M/N/P) to calculate the `SYSCLK` (system clock)
// and `HCLK` (hardware CPU clock) working frequency.
//
// When using `PLL` as clock source, we should calculate `SYSCLK`
// working frequency with the following formula:
//
// PLL_VCO = (HSE_FREQUENCY or HSI_FREQUENCY / PLL_M) * PLL_N
// SYSCLK = PLL_VCO / PLL_P
//                         */
// `VCO` stands for `Voltage-Controlled Oscillator`
//
//
// Tips for picking the right factor value:
//
// 1. `PLL_M`: We can always make it equal to the `HSI_FREQUENCY`
//     or `HSE_FREQUENCY`, then:
//
//     `(HSE_FREQUENCY or HSI_FREQUENCY / PLL_M)` always return `1`
//
//     which more easy to to do the rest calculating. But keep in mind
//     that `PLL_M` have to in the allowed range provided below.
//
//  2. `PLL_P`: We can try start from `2`, then `PLL_M` and `PLL_P`
//      already fixed, only left the `PLL_N` to choose.
//
//  3. If fixed `PLL_M` and `PLL_P` not works, then go to `STM32CubeMX`
//     UI to try the combination.
//

#[cfg(feature = "use-stm32f407g-disc1")]
pub mod clock_source_selecting {
    pub const SYS_CLOCK_MAX_SPEED: u32 = 168_000_000;

    // Use HSI --> PLL as clock source and to max frequency
    pub const HSI_FREQUENCY: u32 = 16_000_000;
    pub const AHB_PRESCALER_FOR_HSI: u32 = 1;
    pub const PLL_M_PRESCALER_FOR_HSI: u32 = 16; // 2 ≤PLLM ≤63
    pub const PLL_N_PRESCALER_FOR_HSI: u32 = 336; // 50 ≤PLLN ≤432
    pub const PLL_P_PRESCALER_FOR_HSI: u32 = 2; // PLLP = 2, 4, 6, or 8
    pub const PLL_Q_PRESCALER_FOR_HSI: u32 = 4; // PLLQ with 2 ≤PLLQ ≤15

    // Use HSE --> PLL as clock source and to max frequency
    pub const HSE_FREQUENCY: u32 = 8_000_000;
    pub const AHB_PRESCALER_FOR_HSE: u32 = 1;
    pub const PLL_M_PRESCALER_FOR_HSE: u32 = 8; // 2 ≤PLLM ≤63
    pub const PLL_N_PRESCALER_FOR_HSE: u32 = 336; // 50 ≤PLLN ≤432
    pub const PLL_P_PRESCALER_FOR_HSE: u32 = 2; // PLLP = 2, 4, 6, or 8
    pub const PLL_Q_PRESCALER_FOR_HSE: u32 = 4; // PLLQ with 2 ≤PLLQ ≤15
}

#[cfg(feature = "use-weact-black-pill")]
pub mod clock_source_selecting {
    pub const SYS_CLOCK_MAX_SPEED: u32 = 100_000_000;

    // Use HSI --> PLL as clock source and to max frequency
    pub const HSI_FREQUENCY: u32 = 16_000_000;
    pub const AHB_PRESCALER_FOR_HSI: u32 = 1;
    pub const PLL_M_PRESCALER_FOR_HSI: u32 = 16; // 2 ≤PLLM ≤63
    pub const PLL_N_PRESCALER_FOR_HSI: u32 = 200; // 50 ≤PLLN ≤432
    pub const PLL_P_PRESCALER_FOR_HSI: u32 = 2; // PLLP = 2, 4, 6, or 8
    pub const PLL_Q_PRESCALER_FOR_HSI: u32 = 4; // PLLQ with 2 ≤PLLQ ≤15

    // Use HSE --> PLL as clock source and to max frequency
    pub const HSE_FREQUENCY: u32 = 25_000_000;
    pub const AHB_PRESCALER_FOR_HSE: u32 = 1;
    pub const PLL_M_PRESCALER_FOR_HSE: u32 = 25; // 2 ≤PLLM ≤63
    pub const PLL_N_PRESCALER_FOR_HSE: u32 = 200; // 50 ≤PLLN ≤432
    pub const PLL_P_PRESCALER_FOR_HSE: u32 = 2; // PLLP = 2, 4, 6, or 8
    pub const PLL_Q_PRESCALER_FOR_HSE: u32 = 4; // PLLQ with 2 ≤PLLQ ≤15
}

// ------ RCC registers address -------------------------------
pub const RCC_CR: u32 = 0x4002_3800; // page 65
pub const RCC_AHB1RSTR: u32 = RCC_CR + 0x10; // page 233
pub const RCC_AHB2RSTR: u32 = RCC_CR + 0x14; // page 236
pub const RCC_AHB3RSTR: u32 = RCC_CR + 0x18; // page 237
pub const RCC_AHB1ENR: u32 = RCC_CR + 0x30; // page 242, 243
pub const RCC_AHB1LPENR: u32 = RCC_CR + 0x50; // Low power (sleep) mode, page 250, 252,
pub const RCC_AHB2ENR: u32 = RCC_CR + 0x34; // page 244
pub const RCC_AHB2LPENR: u32 = RCC_CR + 0x54; // page 252

