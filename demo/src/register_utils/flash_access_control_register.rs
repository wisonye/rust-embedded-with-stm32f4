use core::ptr;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

// ------ Flash access control register (FLASH_ACR) -----------
pub const FLASH_INTERFACE_REGISTER: u32 = 0x4002_3C00; // page 65
pub const FLASH_ACR: u32 = FLASH_INTERFACE_REGISTER; // page 98

// bit0 ~ bit2
pub const FLASH_ACR_LATENCY_START_BIT: u8 = 0;
pub const FLASH_ACR_LATENCY_BITS: u32 = 0b111;

// bit8
pub const FLASH_ACR_PREFETCH_ENABLE_START_BIT: u8 = 8;
pub const FLASH_ACR_PREFETCH_ENABLE_BITS: u32 = 1 << 8;

// bit9
pub const FLASH_ACR_INSTRUCTION_CACHE_ENABLE_START_BIT: u8 = 9;
pub const FLASH_ACR_INSTRUCTION_CACHE_ENABLE_BITS: u32 = 1 << 9;

// bit10
pub const FLASH_ACR_DATA_CACHE_ENABLE_START_BIT: u8 = 10;
pub const FLASH_ACR_DATA_CACHE_ENABLE_BITS: u32 = 1 << 10;

///
#[derive(Debug)]
pub enum FlashReadLatency {
    WrongSetting,
    ZeroWaitState1CpuCycle,
    OneWaitState2CpuCycles,
    TwoWaitState3CpuCycles,
    ThreeWaitState4CpuCycles,
    FourWaitState5CpuCycles,
    FiveWaitState6CpuCycles,
    SixWaitState7CpuCycles,
    SevenWaitState8CpuCycles,
}

/// From `u32` to `FlashReadLatency`
impl From<u32> for FlashReadLatency {
    fn from(value: u32) -> Self {
        match value {
            0 => FlashReadLatency::ZeroWaitState1CpuCycle,
            1 => FlashReadLatency::OneWaitState2CpuCycles,
            2 => FlashReadLatency::TwoWaitState3CpuCycles,
            3 => FlashReadLatency::ThreeWaitState4CpuCycles,
            4 => FlashReadLatency::FourWaitState5CpuCycles,
            5 => FlashReadLatency::FiveWaitState6CpuCycles,
            6 => FlashReadLatency::SixWaitState7CpuCycles,
            7 => FlashReadLatency::SevenWaitState8CpuCycles,
            _ => FlashReadLatency::WrongSetting,
        }
    }
}

///
impl FlashReadLatency {
    pub fn to_register_bits(&self) -> u32 {
        match self {
            Self::WrongSetting => 0b000,
            Self::ZeroWaitState1CpuCycle => 0b000,
            Self::OneWaitState2CpuCycles => 0b001,
            Self::TwoWaitState3CpuCycles => 0b010,
            Self::ThreeWaitState4CpuCycles => 0b011,
            Self::FourWaitState5CpuCycles => 0b100,
            Self::FiveWaitState6CpuCycles => 0b101,
            Self::SixWaitState7CpuCycles => 0b110,
            Self::SevenWaitState8CpuCycles => 0b111,
        }
    }
}

///
pub struct FlashAccessControlRegister {}

/// Alias
pub type FLASH_ACR = FlashAccessControlRegister;

///
impl FlashAccessControlRegister {
    ///
    pub fn set_flash_latency(wait_state: u32) {
        let flash_acr_write_ptr = FLASH_ACR as *mut u32;

        let flash_latency: FlashReadLatency = wait_state.into();
        let flash_latency_bits = flash_latency.to_register_bits();

        // #[cfg(feature = "enable-debug")]
        // {
        // hprintln!("wait_state: {:?}", wait_state);
        // hprintln!("flash_latency: {:?}", flash_latency);
        // hprintln!("flash_latency_bits: {:?}", flash_latency_bits);
        // }

        unsafe {
            ptr::write_volatile(
                flash_acr_write_ptr,
                FLASH_ACR_INSTRUCTION_CACHE_ENABLE_BITS
                    | FLASH_ACR_DATA_CACHE_ENABLE_BITS
                    | FLASH_ACR_PREFETCH_ENABLE_BITS
                    | flash_latency_bits << FLASH_ACR_LATENCY_START_BIT,
            );
        }
    }

    #[cfg(feature = "enable-debug")]
    pub fn print_config() {
        let flash_acr_read_ptr = FLASH_ACR as *const u32;
        let flash_acr_register_value = unsafe { ptr::read_volatile(flash_acr_read_ptr) };

        let flash_latency_bits = flash_acr_register_value & FLASH_ACR_LATENCY_BITS;
        let flash_latency_value = match flash_latency_bits {
            0b000 => FlashReadLatency::ZeroWaitState1CpuCycle,
            0b001 => FlashReadLatency::OneWaitState2CpuCycles,
            0b010 => FlashReadLatency::TwoWaitState3CpuCycles,
            0b011 => FlashReadLatency::ThreeWaitState4CpuCycles,
            0b100 => FlashReadLatency::FourWaitState5CpuCycles,
            0b101 => FlashReadLatency::FiveWaitState6CpuCycles,
            0b110 => FlashReadLatency::SixWaitState7CpuCycles,
            0b111 => FlashReadLatency::SevenWaitState8CpuCycles,
            _ => FlashReadLatency::WrongSetting,
        };

        let prefetch_bit = (flash_acr_register_value
            & FLASH_ACR_PREFETCH_ENABLE_BITS >> FLASH_ACR_PREFETCH_ENABLE_START_BIT);
        let prefetch_enabled = prefetch_bit == 1;

        let instruction_cache_bit = (flash_acr_register_value
            & FLASH_ACR_INSTRUCTION_CACHE_ENABLE_BITS)
            >> FLASH_ACR_INSTRUCTION_CACHE_ENABLE_START_BIT;
        let instruction_cache_enabled = instruction_cache_bit == 1;

        let data_cache_bit = (flash_acr_register_value & FLASH_ACR_DATA_CACHE_ENABLE_BITS)
            >> FLASH_ACR_DATA_CACHE_ENABLE_START_BIT;
        let data_cache_enabled = data_cache_bit == 1;

        let printing_header = "\n[ Flash access control register (FLASH_ACR) ]: \n";
        let _ = hprintln!(
            "{}{}{}{}{}{}",
            printing_header,
            format_args!("FLASH_ACR value: {:#034b}", flash_acr_register_value),
            format_args!(
                "\nFalsh read latency: {:?},\t// bits: {:#05b}",
                flash_latency_value, flash_latency_bits
            ),
            format_args!(
                "\nPrefetch enabled: {:?},\t\t\t\t// bits: {:#02b}",
                prefetch_enabled, prefetch_bit
            ),
            format_args!(
                "\nInstruction cache enabled: {:?},\t\t// bits: {:#2b}",
                instruction_cache_enabled, instruction_cache_bit
            ),
            format_args!(
                "\nData cache enabled: {:?},\t\t\t// bits: {:#2b}",
                data_cache_enabled, data_cache_bit
            )
        );
    }
}
