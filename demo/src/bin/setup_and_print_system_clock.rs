#![allow(warnings)]
#![no_std]
#![no_main]

// We have to use the attribute to locate the module file!
#[path = "../clock_frequency.rs"]
mod clock_frequency;
#[path = "../clock_utils.rs"]
mod clock_utils;
#[path = "../register_utils/flash_access_control_register.rs"]
mod flash_access_control_register;
#[path = "../register_utils/rcc_clock_config_register.rs"]
mod rcc_clock_config_register;
#[path = "../register_utils/rcc_clock_control_register.rs"]
mod rcc_clock_control_register;
#[path = "../rcc_clock_settings.rs"]
mod rcc_clock_settings;
#[path = "../register_utils/rcc_pll_config_register.rs"]
mod rcc_pll_config_register;

use cortex_m_rt::entry;
use panic_semihosting as _;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

use crate::clock_utils::{ClockSource, RccClocks};

///
#[entry]
fn main() -> ! {
    #[cfg(feature = "enable-debug")]
    let _ = hprintln!("STM32F4 setup and print system clock demo is running >>>>>");

    // RccClocks::setup_system_clock(ClockSource::Hsi);
    // RccClocks::setup_system_clock(ClockSource::HsiThroughPll);
    RccClocks::setup_system_clock(ClockSource::HseThroughPll);

    #[cfg(feature = "enable-debug")]
    {
        RccClocks::print_system_clock_info();
    }

    loop {}
}
