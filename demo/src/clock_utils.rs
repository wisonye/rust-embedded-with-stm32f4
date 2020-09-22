use crate::constants::*;
use core::ptr;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

///
pub struct ClockUtils {}

///
impl ClockUtils {
    /// Read u32 value from RCC control register (RCC_CR) and print the clock info.
    #[cfg(feature = "enable-debug")]
    pub fn print_system_clock() {
        let RCC_REGISTER_PTR = RCC_REGISTER as *const u32;
        let RCC_REGISTER_VALUE = unsafe { ptr::read_volatile(RCC_REGISTER_PTR) };

        let hsi_is_on = (RCC_REGISTER_VALUE & RCC_CR_HSI_IS_ON) == RCC_CR_HSI_IS_ON;
        let hsi_is_stable = (RCC_REGISTER_VALUE & RCC_CR_HSI_IS_STABLE) == RCC_CR_HSI_IS_STABLE;
        let hse_is_on = (RCC_REGISTER_VALUE & RCC_CR_HSE_IS_ON) == RCC_CR_HSE_IS_ON;
        let hse_is_stable = (RCC_REGISTER_VALUE & RCC_CR_HSE_IS_STABLE) == RCC_CR_HSE_IS_STABLE;
        let hse_bypass = (RCC_REGISTER_VALUE & RCC_CR_HSE_BYPASS) == RCC_CR_HSE_BYPASS;
        let clock_security_is_on =
            (RCC_REGISTER_VALUE & RCC_CR_CLOCK_SECURITY_IS_ON) == RCC_CR_CLOCK_SECURITY_IS_ON;
        let main_pll_is_on = (RCC_REGISTER_VALUE & RCC_CR_MAIN_PLL_IS_ON) == RCC_CR_MAIN_PLL_IS_ON;
        let main_pll_is_ready =
            (RCC_REGISTER_VALUE & RCC_CR_MAIN_PLL_IS_READY) == RCC_CR_MAIN_PLL_IS_READY;
        let pll_i2s_is_on = (RCC_REGISTER_VALUE & RCC_CR_PLLI2S_IS_ON) == RCC_CR_PLLI2S_IS_ON;
        let pll_i2s_is_ready =
            (RCC_REGISTER_VALUE & RCC_CR_PLLI2S_IS_READY) == RCC_CR_PLLI2S_IS_READY;

        let rcc_register_printing_header = "\n[ RCC_REGISTER ]: \n";

        hprintln!(
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            rcc_register_printing_header,
            format_args!("value: {:034b}", RCC_REGISTER_VALUE),
            format_args!("\nHigh speed internal (HSI) clock enable: {}", &hsi_is_on),
            format_args!(
                "\nHigh speed internal (HSI) clock stable: {}",
                &hsi_is_stable
            ),
            format_args!("\nHigh speed external (HSE) clock enable: {}", &hse_is_on),
            format_args!(
                "\nHigh speed external (HSE) clock stable: {}",
                &hse_is_stable
            ),
            format_args!(
                "\nHSE oscillator bypassed with an external clock: {}",
                &hse_bypass
            ),
            format_args!("\nClock security system enable: {}", &clock_security_is_on),
            format_args!("\nMain PLL (PLL) enable: {}", &main_pll_is_on),
            format_args!("\nMain PLL (PLL) is ready: {}", &main_pll_is_ready),
            format_args!("\nPLLI2S is enable: {}", &pll_i2s_is_on),
            format_args!("\nPLLI2S is ready: {}", &pll_i2s_is_ready)
        );
    }
}
