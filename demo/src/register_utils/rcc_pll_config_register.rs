use crate::rcc_clock_settings::{clock_source_selecting, RCC_CR};
use core::ptr;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

// ------ RCC PLL configuration register (RCC_PLLCFGR) ---------
pub const RCC_PLLCFGR: u32 = RCC_CR + 0x04; // page 226

// bit0 ~ bit5
pub const RCC_PLLCFGR_PLL_M_START_BIT: u8 = 0;
pub const RCC_PLLCFGR_PLL_M_BITS: u32 = 0b111111;
//
// bit6 ~ bit14
pub const RCC_PLLCFGR_PLL_N_START_BIT: u8 = 6;
pub const RCC_PLLCFGR_PLL_N_BITS: u32 = 0b111111111 << 6;

// bit16 ~ bit17
pub const RCC_PLLCFGR_PLL_P_START_BIT: u8 = 16;
pub const RCC_PLLCFGR_PLL_P_BITS: u32 = 0b11 << 16;

// bit24 ~ bit27
pub const RCC_PLLCFGR_PLL_Q_START_BIT: u8 = 24;
pub const RCC_PLLCFGR_PLL_Q_BITS: u32 = 0b1111 << 24;

pub const RCC_PLLCFGR_PLL_SRC_IS_HSE_START_BIT: u32 = 22;
pub const RCC_PLLCFGR_PLL_SRC_IS_HSE_BITS: u32 = 1 << 22;

///
#[derive(Debug)]
pub enum RccPllConfigurationError {
    WrongPllMConfiguration(u16),
    WrongPllNConfiguration(u16),
    WrongPllPConfiguration(u16),
    WrongPllQConfiguration(u16),
}

///
pub struct RccPllConfigurationRegister {}

/// Alias
pub type RccPllCfgr = RccPllConfigurationRegister;

///
impl RccPllConfigurationRegister {
    ///
    pub fn set_pll_mnpq(use_hse: bool) {
        let rcc_pllcfgr_write_ptr = RCC_PLLCFGR as *mut u32;

        let pll_m = if use_hse {
            clock_source_selecting::PLL_M_PRESCALER_FOR_HSE
        } else {
            clock_source_selecting::PLL_M_PRESCALER_FOR_HSI
        };

        let pll_n = if use_hse {
            clock_source_selecting::PLL_N_PRESCALER_FOR_HSE
        } else {
            clock_source_selecting::PLL_N_PRESCALER_FOR_HSI
        };

        let pll_p = if use_hse {
            clock_source_selecting::PLL_P_PRESCALER_FOR_HSE
        } else {
            clock_source_selecting::PLL_P_PRESCALER_FOR_HSI
        };

        let pll_q = if use_hse {
            clock_source_selecting::PLL_Q_PRESCALER_FOR_HSE
        } else {
            clock_source_selecting::PLL_Q_PRESCALER_FOR_HSI
        };

        let mut pll_set_bits = (pll_m << RCC_PLLCFGR_PLL_M_START_BIT)
            | (pll_n << RCC_PLLCFGR_PLL_N_START_BIT)
            | (pll_p << RCC_PLLCFGR_PLL_P_START_BIT)
            | (pll_q << RCC_PLLCFGR_PLL_Q_START_BIT);

        if use_hse {
            pll_set_bits |= RCC_PLLCFGR_PLL_SRC_IS_HSE_BITS;
        }

        // #[cfg(feature = "enable-debug")]
        // {
        // let _ = hprintln!("pll_m: {}", pll_m);
        // let _ = hprintln!("pll_n: {}", pll_n);
        // let _ = hprintln!("pll_p: {}", pll_p);
        // let _ = hprintln!("pll_q: {}", pll_q);
        // }

        unsafe {
            ptr::write_volatile(rcc_pllcfgr_write_ptr, pll_set_bits);
        }
    }

    // pub fn get_pll_m_value() -> Result<u32, RccPllConfigurationError> {
    // let read_ptr = RCC_PLLCFGR as *const u32;
    // let temp_value = unsafe { ptr::read_volatile(read_ptr) & RCC_PLLCFGR_PLL_M_BITS };
    //
    // if temp_value >= 2 && temp_value <= 63 {
    // Ok(temp_value)
    // } else {
    // Err(RccPllConfigurationError::WrongPllMConfiguration(
    // temp_value as u16,
    // ))
    // }
    // }
    //
    // pub fn get_pll_n_value() -> Result<u32, RccPllConfigurationError> {
    // let read_ptr = RCC_PLLCFGR as *const u32;
    // let temp_value = unsafe {
    // (ptr::read_volatile(read_ptr) & RCC_PLLCFGR_PLL_N_BITS) >> RCC_PLLCFGR_PLL_N_START_BIT
    // };
    // if temp_value >= 50 && temp_value <= 432 {
    // Ok(temp_value)
    // } else {
    // Err(RccPllConfigurationError::WrongPllNConfiguration(
    // temp_value as u16,
    // ))
    // }
    // }
    //
    // pub fn get_pll_p_value() -> Result<u32, RccPllConfigurationError> {
    // let read_ptr = RCC_PLLCFGR as *const u32;
    // let temp_value = unsafe {
    // (ptr::read_volatile(read_ptr) & RCC_PLLCFGR_PLL_P_BITS) >> RCC_PLLCFGR_PLL_P_START_BIT
    // };
    // if temp_value == 2 || temp_value == 4 || temp_value == 6 || temp_value == 8 {
    // Ok(temp_value)
    // } else {
    // Err(RccPllConfigurationError::WrongPllPConfiguration(
    // temp_value as u16,
    // ))
    // }
    // }
    //
    // pub fn get_pll_q_value() -> Result<u32, RccPllConfigurationError> {
    // let read_ptr = RCC_PLLCFGR as *const u32;
    // let temp_value = unsafe {
    // (ptr::read_volatile(read_ptr) & RCC_PLLCFGR_PLL_Q_BITS) >> RCC_PLLCFGR_PLL_Q_START_BIT
    // };
    // if temp_value >= 2 && temp_value <= 15 {
    // Ok(temp_value)
    // } else {
    // Err(RccPllConfigurationError::WrongPllQConfiguration(
    // temp_value as u16,
    // ))
    // }
    // }
    //
    // pub fn is_hse_as_pll_src() -> bool {
    // let read_ptr = RCC_PLLCFGR as *const u32;
    // unsafe {
    // ptr::read_volatile(read_ptr) & RCC_PLLCFGR_PLL_SRC_IS_HSE == RCC_PLLCFGR_PLL_SRC_IS_HSE
    // }
    // }

    #[cfg(feature = "enable-debug")]
    pub fn print_config() {
        let read_ptr = RCC_PLLCFGR as *const u32;
        let cfg_register_value = unsafe { ptr::read_volatile(read_ptr) };

        let temp_m_value = cfg_register_value & RCC_PLLCFGR_PLL_M_BITS;
        let pll_m_value = if temp_m_value >= 2 && temp_m_value <= 63 {
            Ok(temp_m_value)
        } else {
            Err(RccPllConfigurationError::WrongPllMConfiguration(
                temp_m_value as u16,
            ))
        };

        let temp_n_value =
            (cfg_register_value & RCC_PLLCFGR_PLL_N_BITS) >> RCC_PLLCFGR_PLL_N_START_BIT;
        let pll_n_value = if temp_n_value >= 50 && temp_n_value <= 432 {
            Ok(temp_n_value)
        } else {
            Err(RccPllConfigurationError::WrongPllNConfiguration(
                temp_n_value as u16,
            ))
        };

        let temp_p_value =
            (cfg_register_value & RCC_PLLCFGR_PLL_P_BITS) >> RCC_PLLCFGR_PLL_P_START_BIT;
        let pll_p_value =
            if temp_p_value == 2 || temp_p_value == 4 || temp_p_value == 6 || temp_p_value == 8 {
                Ok(temp_p_value)
            } else {
                Err(RccPllConfigurationError::WrongPllPConfiguration(
                    temp_p_value as u16,
                ))
            };

        let temp_q_value =
            (cfg_register_value & RCC_PLLCFGR_PLL_Q_BITS) >> RCC_PLLCFGR_PLL_Q_START_BIT;
        let pll_q_value = if temp_q_value >= 2 && temp_q_value <= 15 {
            Ok(temp_q_value)
        } else {
            Err(RccPllConfigurationError::WrongPllQConfiguration(
                temp_q_value as u16,
            ))
        };

        let pll_source = (cfg_register_value & RCC_PLLCFGR_PLL_SRC_IS_HSE_BITS)
            >> RCC_PLLCFGR_PLL_SRC_IS_HSE_START_BIT;
        let pll_source_is_hse = pll_source == 1;
        let pll_source_desc = if pll_source_is_hse { "HSE" } else { "HSI" };

        let printing_header = "\n[ RCC PLL configuration register (RCC_PLLCFGR) ]: \n";
        let _ = hprintln!(
            "{}{}{}{}{}{}{}",
            printing_header,
            format_args!("RCC_PLLCFGR value: {:#034b}", cfg_register_value),
            format_args!(
                "\nMain PLL M: {:?},\t// bits: {:#08b}",
                pll_m_value, temp_m_value
            ),
            format_args!(
                "\nMain PLL N: {:?},\t// bits: {:#011b}",
                pll_n_value, temp_n_value
            ),
            format_args!(
                "\nMain PLL P: {:?},\t// bits: {:#04b}",
                pll_p_value, temp_p_value
            ),
            format_args!(
                "\nMain PLL q: {:?},\t// bits: {:#06b}",
                pll_q_value, temp_q_value
            ),
            format_args!(
                "\nMain PLL source: {:?},\t// bits: {:#02b}",
                pll_source_desc, pll_source
            )
        );
    }
}
