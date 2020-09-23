// use core::ptr;
// use crate::constants::*;
use crate::rcc_clock_control_register::RccClockControlRegister;
use crate::rcc_pll_config_register::RccPllConfigurationRegister;

// #[cfg(feature = "enable-debug")]
// use cortex_m_semihosting::hprintln;

///
pub struct ClockUtils {}

///
impl ClockUtils {
    /// Read u32 value from RCC control register (RCC_CR) and print the clock info.
    #[cfg(feature = "enable-debug")]
    pub fn print_system_clock_info() {
        RccClockControlRegister::print_config();
        RccPllConfigurationRegister::print_config();
    }
}
