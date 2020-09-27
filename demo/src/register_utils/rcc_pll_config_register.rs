use crate::rcc_clock_settings::RCC_CR;
use core::ptr;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

// ------ RCC PLL configuration register (RCC_PLLCFGR) ---------
pub const RCC_PLLCFGR: u32 = RCC_CR + 0x04; // page 226

// bit0 ~ bit5
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

pub const RCC_PLLCFGR_PLL_SRC_IS_HSE: u32 = 1 << 22;

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
pub type RCC_PLLCFGR = RccPllConfigurationRegister;

///
impl RccPllConfigurationRegister {
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
        let _ = hprintln!("\n\ttemp_m_value: {:#018b}", temp_m_value);
        let pll_m_value = if temp_m_value >= 2 && temp_m_value <= 63 {
            Ok(temp_m_value)
        } else {
            Err(RccPllConfigurationError::WrongPllMConfiguration(
                temp_m_value as u16,
            ))
        };

        let temp_n_value =
            (cfg_register_value & RCC_PLLCFGR_PLL_N_BITS) >> RCC_PLLCFGR_PLL_N_START_BIT;
        let _ = hprintln!("\ttemp_n_value: {:#018b}", temp_n_value);
        let pll_n_value = if temp_n_value >= 50 && temp_n_value <= 432 {
            Ok(temp_n_value)
        } else {
            Err(RccPllConfigurationError::WrongPllNConfiguration(
                temp_n_value as u16,
            ))
        };

        let temp_p_value =
            (cfg_register_value & RCC_PLLCFGR_PLL_P_BITS) >> RCC_PLLCFGR_PLL_P_START_BIT;
        let _ = hprintln!("\ttemp_p_value: {:#018b}", temp_p_value);
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
        let _ = hprintln!("\ttemp_q_value: {:#018b}", temp_q_value);
        let pll_q_value = if temp_q_value >= 2 && temp_q_value <= 15 {
            Ok(temp_q_value)
        } else {
            Err(RccPllConfigurationError::WrongPllQConfiguration(
                temp_q_value as u16,
            ))
        };

        let pll_source =
            cfg_register_value & RCC_PLLCFGR_PLL_SRC_IS_HSE == RCC_PLLCFGR_PLL_SRC_IS_HSE;
        let pll_source_desc = if pll_source { "HSE" } else { "HSI" };

        let printing_header = "\n[ RCC PLL configuration register (RCC_PLLCFGR) ]: \n";
        let _ = hprintln!(
            "{}{}{}{}{}{}{}",
            printing_header,
            format_args!("RCC_PLLCFGR value: {:#034b}", cfg_register_value),
            format_args!("\nMain PLL M: {:?}", pll_m_value),
            format_args!("\nMain PLL N: {:?}", pll_n_value),
            format_args!("\nMain PLL P: {:?}", pll_p_value),
            format_args!("\nMain PLL q: {:?}", pll_q_value),
            format_args!("\nMain PLL source: {:?}", pll_source_desc)
        );
    }
}
