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
#[path = "../register_utils/system_tick_timer_register.rs"]
mod system_tick_timer_register;

use cortex_m_rt::entry;
use panic_semihosting as _;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

use crate::clock_utils::{ClockSource, RccClocks};
use system_tick_timer_register::SystemTickTimer;

///
#[entry]
fn main() -> ! {
    #[cfg(feature = "enable-debug")]
    let _ = hprintln!("STM32F4 setup and print system clock demo is running >>>>>");

    // let rcc_clocks = RccClocks::setup_system_clock(ClockSource::Hsi);
    // let rcc_clocks = RccClocks::setup_system_clock(ClockSource::HsiThroughPll);
    let rcc_clocks = RccClocks::setup_system_clock(ClockSource::HseThroughPll);

    let clock_speed = rcc_clocks.get_cpu_clock_frequency_in_hertz();
    SystemTickTimer::enable(clock_speed);

    #[cfg(feature = "enable-debug")]
    {
        // RccClocks::print_system_clock_info();
        hprintln!(
            "CPU clock frequency: {}",
            rcc_clocks.get_cpu_clock_frequency_in_hertz()
        );
        SystemTickTimer::print_config();
    }

    let delay_value = 1000;

    loop {
        // #[cfg(feature = "enable-debug")]
        // {
        // hprintln!(
        // "System tick timer current countdown value: {}",
        // SystemTickTimer::get_current_countdown_value()
        // );
        // }
        //

        SystemTickTimer::delay(clock_speed, delay_value);
        // SystemTickTimer::delay_for_ms(clock_speed, 500);
        hprintln!("Delayed {} ms", delay_value);
    }
}
