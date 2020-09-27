use crate::rcc_clock_settings::{clock_source_selecting, RCC_CR};
use core::ptr;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

// ------ RCC clock configuration register (RCC_CFGR) ---------
pub const RCC_CGFCR: u32 = RCC_CR + 0x08; // page 228

// bit0 ~ bit1
pub const RCC_CFGR_SYS_CLOCK_SWITCH_START_BIT: u8 = 0;
pub const RCC_CFGR_SYS_CLOCK_SWITCH_BITS: u32 = 0b11;

// bit2 ~ bit3
pub const RCC_CFGR_SYS_CLOCK_SWITCH_STATUS_START_BIT: u8 = 2;
pub const RCC_CFGR_SYS_CLOCK_SWITCH_STATUS_BITS: u32 = 0b11 << 2;

// bit4 ~ bit7
pub const RCC_CFGR_AHB_PRESCALER_START_BIT: u8 = 4;
pub const RCC_CFGR_AHB_PRESCALER_BITS: u32 = 0b1111 << 4;

// bit10 ~ bit12
pub const RCC_CFGR_APB1_LOW_SPEED_PRESCALER_START_BIT: u8 = 10;
pub const RCC_CFGR_APB1_LOW_SPEED_PRESCALER_BITS: u32 = 0b111 << 10;

// bit13 ~ bit15
pub const RCC_CFGR_APB1_HIGH_SPEED_PRESCALER_START_BIT: u8 = 13;
pub const RCC_CFGR_APB1_HIGH_SPEED_PRESCALER_BITS: u32 = 0b111 << 13;
///
#[derive(Debug)]
pub enum RccSystemClockSwtich {
    NotAllowed,
    HsiSelectedAsSytemClock,
    HseSelectedAsSytemClock,
    PllSelectedAsSytemClock,
}

impl RccSystemClockSwtich {
    pub fn to_register_bits(&self) -> u32 {
        match self {
            RccSystemClockSwtich::NotAllowed => 0b11,
            RccSystemClockSwtich::HsiSelectedAsSytemClock => 0b00,
            RccSystemClockSwtich::HseSelectedAsSytemClock => 0b01,
            RccSystemClockSwtich::PllSelectedAsSytemClock => 0b10,
        }
    }
}

///
#[derive(Debug)]
pub enum RccSystemClockSwtichStatus {
    NotApplicable,
    HsiUsedAsSytemClock,
    HseUsedAsSytemClock,
    PllUsedAsSytemClock,
}

#[derive(Debug)]
pub enum RccAhbPrescaler {
    SystemClockNotDivided,
    SystemClockDividedBy2,
    SystemClockDividedBy4,
    SystemClockDividedBy8,
    SystemClockDividedBy16,
    SystemClockDividedBy64,
    SystemClockDividedBy128,
    SystemClockDividedBy256,
    SystemClockDividedBy512,
}

/// From `RccAhbPrescaler` to `u32`
impl From<RccAhbPrescaler> for u32 {
    fn from(value: RccAhbPrescaler) -> Self {
        match value {
            RccAhbPrescaler::SystemClockNotDivided => 1,
            RccAhbPrescaler::SystemClockDividedBy2 => 2,
            RccAhbPrescaler::SystemClockDividedBy4 => 4,
            RccAhbPrescaler::SystemClockDividedBy8 => 8,
            RccAhbPrescaler::SystemClockDividedBy16 => 16,
            RccAhbPrescaler::SystemClockDividedBy64 => 64,
            RccAhbPrescaler::SystemClockDividedBy128 => 128,
            RccAhbPrescaler::SystemClockDividedBy256 => 256,
            RccAhbPrescaler::SystemClockDividedBy512 => 512,
        }
    }
}

/// From `u32` to `RccAhbPrescaler`
impl From<u32> for RccAhbPrescaler {
    fn from(value: u32) -> Self {
        match value {
            2 => RccAhbPrescaler::SystemClockDividedBy2,
            4 => RccAhbPrescaler::SystemClockDividedBy4,
            8 => RccAhbPrescaler::SystemClockDividedBy8,
            16 => RccAhbPrescaler::SystemClockDividedBy16,
            64 => RccAhbPrescaler::SystemClockDividedBy64,
            128 => RccAhbPrescaler::SystemClockDividedBy128,
            256 => RccAhbPrescaler::SystemClockDividedBy256,
            512 => RccAhbPrescaler::SystemClockDividedBy512,
            _ => RccAhbPrescaler::SystemClockNotDivided,
        }
    }
}

impl RccAhbPrescaler {
    fn to_register_bits(&self) -> u32 {
        match self {
            RccAhbPrescaler::SystemClockNotDivided => 0b0000,
            RccAhbPrescaler::SystemClockDividedBy2 => 0b1000,
            RccAhbPrescaler::SystemClockDividedBy4 => 0b1001,
            RccAhbPrescaler::SystemClockDividedBy8 => 0b1010,
            RccAhbPrescaler::SystemClockDividedBy16 => 0b1011,
            RccAhbPrescaler::SystemClockDividedBy64 => 0b1100,
            RccAhbPrescaler::SystemClockDividedBy128 => 0b1101,
            RccAhbPrescaler::SystemClockDividedBy256 => 0b1110,
            RccAhbPrescaler::SystemClockDividedBy512 => 0b1111,
        }
    }
}

#[derive(Debug)]
pub enum RccApbPrescaler {
    AhbClockNotDivided,
    AhbClockDividedBy2,
    AhbClockDividedBy4,
    AhbClockDividedBy8,
    AhbClockDividedBy16,
}

/// From `RccApbPrescaler` to `u32`
impl From<RccApbPrescaler> for u32 {
    fn from(value: RccApbPrescaler) -> Self {
        match value {
            RccApbPrescaler::AhbClockNotDivided => 1,
            RccApbPrescaler::AhbClockDividedBy2 => 2,
            RccApbPrescaler::AhbClockDividedBy4 => 4,
            RccApbPrescaler::AhbClockDividedBy8 => 8,
            RccApbPrescaler::AhbClockDividedBy16 => 16,
        }
    }
}

/// From `u32` to `RccApbPrescaler`
impl From<u32> for RccApbPrescaler {
    fn from(value: u32) -> Self {
        match value {
            2 => RccApbPrescaler::AhbClockDividedBy2,
            4 => RccApbPrescaler::AhbClockDividedBy4,
            8 => RccApbPrescaler::AhbClockDividedBy8,
            16 => RccApbPrescaler::AhbClockDividedBy16,
            _ => RccApbPrescaler::AhbClockNotDivided,
        }
    }
}

impl RccApbPrescaler {
    fn to_register_bits(&self) -> u32 {
        match self {
            RccApbPrescaler::AhbClockNotDivided => 0b000,
            RccApbPrescaler::AhbClockDividedBy2 => 0b100,
            RccApbPrescaler::AhbClockDividedBy4 => 0b101,
            RccApbPrescaler::AhbClockDividedBy8 => 0b110,
            RccApbPrescaler::AhbClockDividedBy16 => 0b111,
        }
    }
}

///
pub struct RccClockConfigurationRegister {}

/// Alias
pub type RCC_CFGR = RccClockConfigurationRegister;

///
impl RccClockConfigurationRegister {
    ///
    pub fn set_bus_prescaler(use_hse: bool) {
        let rcc_cfgr_write_ptr = RCC_CGFCR as *mut u32;

        let ahb_prescaler: RccAhbPrescaler = if use_hse {
            clock_source_selecting::AHB_PRESCALER_FOR_HSE
        } else {
            clock_source_selecting::AHB_PRESCALER_FOR_HSI
        }
        .into();
        let ahb_prescaler_bits = ahb_prescaler.to_register_bits();

        let apb1_prescaler: RccApbPrescaler = if use_hse {
            clock_source_selecting::APB1_PRESCALER_FOR_HSE
        } else {
            clock_source_selecting::APB1_PRESCALER_FOR_HSI
        }
        .into();
        let apb1_prescaler_bits = apb1_prescaler.to_register_bits();

        let apb2_prescaler: RccApbPrescaler = if use_hse {
            clock_source_selecting::APB2_PRESCALER_FOR_HSE
        } else {
            clock_source_selecting::APB2_PRESCALER_FOR_HSI
        }
        .into();
        let apb2_prescaler_bits = apb2_prescaler.to_register_bits();

        // #[cfg(feature = "enable-debug")]
        // {
        // hprintln!("ahb_prescaler: {:?}", ahb_prescaler);
        // hprintln!("ahb_prescaler_bits: {}", ahb_prescaler_bits);
        // hprintln!("ahb_prescaler_bits: {:#06b}", ahb_prescaler_bits);
        // hprintln!("apb1_prescaler: {:?}", apb1_prescaler);
        // hprintln!("apb1_prescaler_bits: {}", apb1_prescaler_bits);
        // hprintln!("apb1_prescaler_bits: {:#04b}", apb1_prescaler_bits);
        // hprintln!("apb2_prescaler: {:?}", apb2_prescaler);
        // hprintln!("apb2_prescaler_bits: {}", apb2_prescaler_bits);
        // hprintln!("apb2_prescaler_bits: {:#04b}", apb2_prescaler_bits);
        // }

        unsafe {
            ptr::write_volatile(
                rcc_cfgr_write_ptr,
                (ahb_prescaler_bits << RCC_CFGR_AHB_PRESCALER_START_BIT)
                    | (apb1_prescaler_bits << RCC_CFGR_APB1_LOW_SPEED_PRESCALER_START_BIT)
                    | (apb2_prescaler_bits << RCC_CFGR_APB1_HIGH_SPEED_PRESCALER_START_BIT),
            );
        }

        // Wait for the new prescalers to kick in
        // "The clocks are divided with the new prescaler factor from 1 to 16 AHB cycles after write"
        cortex_m::asm::delay(16);
    }

    ///
    pub fn switch_clock_source_and_wait_for_stable(clock_source: RccSystemClockSwtich) {
        let clock_source_bits = clock_source.to_register_bits();
        #[cfg(feature = "enable-debug")]
        {
            let _ = hprintln!("clock_source: {:?}", clock_source);
            let _ = hprintln!("clock_source_bits: {:#04b}", clock_source_bits);
        }

        let rcc_cfgr_write_ptr = RCC_CGFCR as *mut u32;
        let rcc_cfgr_read_ptr = RCC_CGFCR as *const u32;
        unsafe {
            ptr::write_volatile(
                rcc_cfgr_write_ptr,
                clock_source_bits << RCC_CFGR_SYS_CLOCK_SWITCH_START_BIT,
            );
        }

        let mut still_not_stable = true;
        while still_not_stable {
            let temp_register_value = unsafe { ptr::read_volatile(rcc_cfgr_read_ptr) };
            let clock_switch_status_bits = (temp_register_value
                & RCC_CFGR_SYS_CLOCK_SWITCH_STATUS_BITS)
                >> RCC_CFGR_SYS_CLOCK_SWITCH_STATUS_START_BIT;
            still_not_stable = clock_switch_status_bits != clock_source_bits;

            #[cfg(feature = "enable-debug")]
            {
                let _ = hprintln!("clock_switch_status_bits: {:#04b}", clock_switch_status_bits);
                let _ = hprintln!("still_not_stable: {}", still_not_stable);
                let _ = hprintln!("Waiting for clock switch become stable>>>>>");
            }
        }
    }

    #[cfg(feature = "enable-debug")]
    pub fn print_config() {
        let rcc_sys_cfg_ptr = RCC_CGFCR as *const u32;
        let rcc_sys_cfg_register_value = unsafe { ptr::read_volatile(rcc_sys_cfg_ptr) };

        let clock_switch_bits = rcc_sys_cfg_register_value & RCC_CFGR_SYS_CLOCK_SWITCH_BITS;
        let _ = hprintln!("\n\tclock_switch_bits: {:#04b}", clock_switch_bits);
        let clock_switch_value = match clock_switch_bits {
            0b00 => RccSystemClockSwtich::HsiSelectedAsSytemClock,
            0b01 => RccSystemClockSwtich::HseSelectedAsSytemClock,
            0b10 => RccSystemClockSwtich::PllSelectedAsSytemClock,
            0b11 => RccSystemClockSwtich::NotAllowed,
            _ => RccSystemClockSwtich::NotAllowed,
        };

        let clock_switch_status_bits = (rcc_sys_cfg_register_value
            & RCC_CFGR_SYS_CLOCK_SWITCH_STATUS_BITS)
            >> RCC_CFGR_SYS_CLOCK_SWITCH_STATUS_START_BIT;
        let _ = hprintln!(
            "\tclock_switch_status_bits: {:#04b}",
            clock_switch_status_bits
        );
        let clock_switch_status_value = match clock_switch_status_bits {
            0b00 => RccSystemClockSwtichStatus::HsiUsedAsSytemClock,
            0b01 => RccSystemClockSwtichStatus::HseUsedAsSytemClock,
            0b10 => RccSystemClockSwtichStatus::PllUsedAsSytemClock,
            0b11 => RccSystemClockSwtichStatus::NotApplicable,
            _ => RccSystemClockSwtichStatus::NotApplicable,
        };

        let ahb_prescaler_bits = (rcc_sys_cfg_register_value & RCC_CFGR_AHB_PRESCALER_BITS)
            >> RCC_CFGR_AHB_PRESCALER_START_BIT;
        let _ = hprintln!("\tahb_prescaler_bits: {:#06b}", ahb_prescaler_bits);
        let ahb_prescaler_value: u32 = match ahb_prescaler_bits {
            0b1000 => RccAhbPrescaler::SystemClockDividedBy2,
            0b1001 => RccAhbPrescaler::SystemClockDividedBy4,
            0b1010 => RccAhbPrescaler::SystemClockDividedBy8,
            0b1011 => RccAhbPrescaler::SystemClockDividedBy16,
            0b1100 => RccAhbPrescaler::SystemClockDividedBy64,
            0b1101 => RccAhbPrescaler::SystemClockDividedBy128,
            0b1110 => RccAhbPrescaler::SystemClockDividedBy256,
            0b1111 => RccAhbPrescaler::SystemClockDividedBy512,
            _ => RccAhbPrescaler::SystemClockNotDivided,
        }
        .into();

        let apb1_low_speed_prescaler_bits = (rcc_sys_cfg_register_value
            & RCC_CFGR_APB1_LOW_SPEED_PRESCALER_BITS)
            >> RCC_CFGR_APB1_LOW_SPEED_PRESCALER_START_BIT;
        let _ = hprintln!(
            "\tapb1_low_speed_prescaler_bits: {:#05b}",
            apb1_low_speed_prescaler_bits
        );
        let apb1_low_speed_prescaler_value: u32 = match apb1_low_speed_prescaler_bits {
            0b100 => RccApbPrescaler::AhbClockDividedBy2,
            0b101 => RccApbPrescaler::AhbClockDividedBy4,
            0b110 => RccApbPrescaler::AhbClockDividedBy8,
            0b111 => RccApbPrescaler::AhbClockDividedBy16,
            _ => RccApbPrescaler::AhbClockNotDivided,
        }
        .into();

        let apb2_high_speed_prescaler_bits = (rcc_sys_cfg_register_value
            & RCC_CFGR_APB1_HIGH_SPEED_PRESCALER_BITS)
            >> RCC_CFGR_APB1_HIGH_SPEED_PRESCALER_START_BIT;
        let _ = hprintln!(
            "\tapb2_high_speed_prescaler_bits: {:#05b}",
            apb2_high_speed_prescaler_bits
        );
        let apb2_high_speed_prescaler_value: u32 = match apb2_high_speed_prescaler_bits {
            0b100 => RccApbPrescaler::AhbClockDividedBy2,
            0b101 => RccApbPrescaler::AhbClockDividedBy4,
            0b110 => RccApbPrescaler::AhbClockDividedBy8,
            0b111 => RccApbPrescaler::AhbClockDividedBy16,
            _ => RccApbPrescaler::AhbClockNotDivided,
        }
        .into();

        let printing_header = "\n[ RCC clock configuration register (RCC_CFGR) ]: \n";
        let _ = hprintln!(
            "{}{}{}{}{}{}{}",
            printing_header,
            format_args!("value: {:034b}", rcc_sys_cfg_register_value),
            format_args!("\nSystem clock selected: {:?}", clock_switch_value),
            format_args!("\nSystem clock status: {:?}", clock_switch_status_value),
            format_args!("\nAHB prescaler: {:?}", ahb_prescaler_value),
            format_args!(
                "\nAPB low speed prescaler (APB1): {:?}",
                apb1_low_speed_prescaler_value
            ),
            format_args!(
                "\nAPB high speed prescaler (APB2): {:?}",
                apb2_high_speed_prescaler_value
            ),
        );
    }
}
