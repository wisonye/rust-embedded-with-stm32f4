#![no_std]
#![no_main]

// We have to use the attribute to locate the module file!
#[path = "../clock_frequency.rs"]
mod clock_frequency;
#[path = "../clock_utils.rs"]
mod clock_utils;
#[path = "../register_utils/rcc_clock_config_register.rs"]
mod rcc_clock_config_register;
#[path = "../register_utils/rcc_clock_control_register.rs"]
mod rcc_clock_control_register;
#[path = "../rcc_clock_settings.rs"]
mod rcc_clock_settings;
#[path = "../register_utils/rcc_pll_config_register.rs"]
mod rcc_pll_config_register;

use cortex_m_rt::entry;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

use panic_semihosting as _;

use crate::clock_utils::{ClockSource, RccClocks};

use crate::hal::{
    prelude::*,
    stm32, // The `stm32` should means `stm32f407` which enable by the `features` in `.toml`
};

// This is very important!!!
use stm32f4xx_hal as hal;

// Import from `stm32f4xx_hal`
use hal::rcc::{Clocks, Rcc};

// #[cfg(feature = "enable-debug")]
// fn print_clocks(clocks: &Clocks) {
// fn get_megahertz_is_possible(frequency: u32) -> u32 {
// if frequency > 1_000_000 {
// frequency / 1_000_000
// } else {
// frequency
// }
// }
//
// let _ = hprintln!("\n[ Clocks Frequency ]:\nAHB1 Frequency: {}MHz\nAPB1 Frequency: {}MHz\nAPB2 Frequency: {}MHz\nAPB1 Prescaler: {}\nAPB2 Prescaler: {}\nSystem Core Frequency: {}Mhz\nPLL48 clock line Frequency: {}MHz\nPLL48 clock is within USB specifications: {}\n",
// get_megahertz_is_possible(clocks.hclk().0),
// get_megahertz_is_possible(clocks.pclk1().0),
// get_megahertz_is_possible(clocks.pclk2().0),
// clocks.ppre1(),
// clocks.ppre2(),
// get_megahertz_is_possible(clocks.sysclk().0),
// match clocks.pll48clk() {
// Some(value) => value.0,
// None => 0
// },
// clocks.is_pll48clk_valid()
// );
// }

///
#[entry]
fn main() -> ! {
    #[cfg(feature = "enable-debug")]
    let _ = hprintln!("STM32F4 setup and print system clock demo is running >>>>>");

    let stm32407_peripherals = stm32::Peripherals::take().unwrap();
    // let cortex_m_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock. We want to run at 16Mhz for this one.
    let constrained_rcc_peripheral: Rcc = stm32407_peripherals.RCC.constrain();
    // let clocks: Clocks = constrained_rcc_peripheral.cfgr.sysclk(16.mhz()).freeze();
    // let clocks: Clocks = constrained_rcc_peripheral.cfgr.use_hse(25.mhz()).freeze();

    #[cfg(feature = "enable-debug")]
    {
        // print_clocks(&clocks);
        // RccClocks::setup_system_clock(ClockSource::Hsi);
        // RccClocks::setup_system_clock(ClockSource::HsiThroughPll);
        RccClocks::setup_system_clock(ClockSource::HseThroughPll);
        // RccClocks::print_system_clock_info();
    }

    loop {}
}
