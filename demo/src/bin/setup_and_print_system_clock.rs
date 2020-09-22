#![no_std]
#![no_main]

// We have to use the attribute to locate the mod file!
#[path = "../constants.rs"]
mod constants;
#[path = "../clock_utils.rs"]
mod clock_utils;
#[path = "../register_utils.rs"]
mod register_utils;

use cortex_m_rt::entry;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

use panic_semihosting as _;

use crate::clock_utils::ClockUtils;
use crate::register_utils::RccPllConfigurationRegister;

///

#[entry]
fn main() -> ! {
    #[cfg(feature = "enable-debug")]
    {
        let _ = hprintln!("STM32F4 setup and print system clock demo is running >>>>>");

        ClockUtils::print_system_clock_info();

        RccPllConfigurationRegister::print_config();
    }

    loop {}
}
