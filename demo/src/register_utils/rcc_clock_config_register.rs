use crate::rcc_clock_settings::RCC_CR;
use core::ptr;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

// ------ RCC clock configuration register (RCC_CFGR) ---------
pub const RCC_CGFCR: u32 = RCC_CR + 0x08; // page 228

// bit0 ~ bit1
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

impl From<RccAhbPrescaler> for u16 {
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

#[derive(Debug)]
pub enum RccApbPrescaler {
    AhbClockNotDivided,
    AhbClockDividedBy2,
    AhbClockDividedBy4,
    AhbClockDividedBy8,
    AhbClockDividedBy16,
}

impl From<RccApbPrescaler> for u16 {
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

///
pub struct RccClockConfigurationRegister {}

/// Alias
pub type RCC_CFGR = RccClockConfigurationRegister;

///
impl RccClockConfigurationRegister {
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
        let ahb_prescaler_value: u16 = match ahb_prescaler_bits {
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
        let apb1_low_speed_prescaler_value: u16 = match apb1_low_speed_prescaler_bits {
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
        let apb2_high_speed_prescaler_value: u16 = match apb2_high_speed_prescaler_bits {
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
