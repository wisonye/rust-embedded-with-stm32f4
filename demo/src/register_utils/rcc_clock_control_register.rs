use crate::rcc_clock_settings::RCC_CR;
use core::ptr;
use cortex_m::asm::nop;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

// ------ RCC clock control register (RCC_CR), page 224 -------
pub const RCC_CR_HSI_IS_ON: u32 = 1u32;
pub const RCC_CR_HSI_IS_STABLE: u32 = 1 << 1;
pub const RCC_CR_HSE_IS_ON: u32 = 1 << 16;
pub const RCC_CR_HSE_IS_STABLE: u32 = 1 << 17;
pub const RCC_CR_HSE_BYPASS: u32 = 1 << 18;
pub const RCC_CR_CLOCK_SECURITY_IS_ON: u32 = 1 << 19;
pub const RCC_CR_MAIN_PLL_IS_ON: u32 = 1 << 24;
pub const RCC_CR_MAIN_PLL_IS_READY: u32 = 1 << 25;
pub const RCC_CR_PLLI2S_IS_ON: u32 = 1 << 26;
pub const RCC_CR_PLLI2S_IS_READY: u32 = 1 << 27;

///
pub struct RccClockControlRegister {}

/// Alias
pub type RccCr = RccClockControlRegister;

///
impl RccClockControlRegister {
    ///
    pub fn reset() {
        unsafe {
            ptr::write_volatile(RCC_CR as *mut u32, 0x0000_0083);
        }
    }

    /// Make sure call somewhere call `reset()` before calling this function!!!
    pub fn enable_hse_as_clock_source_and_wait_for_it_stable() {
        let rcc_cr_write_ptr = RCC_CR as *mut u32;
        let rcc_cr_read_ptr = RCC_CR as *const u32;
        unsafe {
            ptr::write_volatile(rcc_cr_write_ptr, RCC_CR_HSE_IS_ON);

            while (ptr::read_volatile(rcc_cr_read_ptr) & RCC_CR_HSE_IS_ON) != RCC_CR_HSE_IS_ON {
                #[cfg(feature = "enable-debug")]
                let _ = hprintln!("Waiting for HSE to become stable>>>>>");

                nop();
            }
        }
    }

    /// Make sure call somewhere call `reset()` before calling this function!!!
    pub fn enable_pll_and_wait_for_it_stable() {
        let rcc_cr_write_ptr = RCC_CR as *mut u32;
        let rcc_cr_read_ptr = RCC_CR as *const u32;
        unsafe {
            ptr::write_volatile(rcc_cr_write_ptr, RCC_CR_MAIN_PLL_IS_ON);

            while (ptr::read_volatile(rcc_cr_read_ptr) & RCC_CR_MAIN_PLL_IS_READY) != RCC_CR_MAIN_PLL_IS_READY {
                #[cfg(feature = "enable-debug")]
                let _ = hprintln!("Waiting for Main PLL to become stable>>>>>");

                nop();
            }
        }
    }

    #[cfg(feature = "enable-debug")]
    pub fn print_config() {
        let rcc_register_ptr = RCC_CR as *const u32;
        let rcc_register_value = unsafe { ptr::read_volatile(rcc_register_ptr) };

        let hsi_is_on = (rcc_register_value & RCC_CR_HSI_IS_ON) == RCC_CR_HSI_IS_ON;
        let hsi_is_stable = (rcc_register_value & RCC_CR_HSI_IS_STABLE) == RCC_CR_HSI_IS_STABLE;
        let hse_is_on = (rcc_register_value & RCC_CR_HSE_IS_ON) == RCC_CR_HSE_IS_ON;
        let hse_is_stable = (rcc_register_value & RCC_CR_HSE_IS_STABLE) == RCC_CR_HSE_IS_STABLE;
        let hse_bypass = (rcc_register_value & RCC_CR_HSE_BYPASS) == RCC_CR_HSE_BYPASS;
        let clock_security_is_on =
            (rcc_register_value & RCC_CR_CLOCK_SECURITY_IS_ON) == RCC_CR_CLOCK_SECURITY_IS_ON;
        let main_pll_is_on = (rcc_register_value & RCC_CR_MAIN_PLL_IS_ON) == RCC_CR_MAIN_PLL_IS_ON;
        let main_pll_is_ready =
            (rcc_register_value & RCC_CR_MAIN_PLL_IS_READY) == RCC_CR_MAIN_PLL_IS_READY;
        let pll_i2s_is_on = (rcc_register_value & RCC_CR_PLLI2S_IS_ON) == RCC_CR_PLLI2S_IS_ON;
        let pll_i2s_is_ready =
            (rcc_register_value & RCC_CR_PLLI2S_IS_READY) == RCC_CR_PLLI2S_IS_READY;

        let rcc_register_printing_header = "\n[ RCC clock control register (RCC_CR) ]: \n";

        let _ = hprintln!(
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            rcc_register_printing_header,
            format_args!("value: {:034b}", rcc_register_value),
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
