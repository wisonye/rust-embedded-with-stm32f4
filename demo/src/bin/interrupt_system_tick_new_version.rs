#![allow(warnings)]
#![no_std]
#![no_main]

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

#[entry]
fn main() -> ! {
    #[cfg(feature = "enable-debug")]
    let _ = hprintln!("STM32F4 system tick interrput demo is running >>>>>");

    let rcc_clock = RccClocks::setup_system_clock(ClockSource::HseThroughPll);
    SystemTickTimer::enable(rcc_clock.get_cpu_clock_frequency_in_hertz(), true);

    #[cfg(feature = "enable-debug")]
    {
        // RccClocks::print_system_clock_info();
        SystemTickTimer::print_config();
    }

    loop {}
}

#[export_name = "SysTick"]
pub unsafe extern "C" fn system_tick_exception_handler_wrapper() {
    system_tick_exception_handler({
        static mut current_past_milliseconds_count: u32 = 0u32;
        &mut current_past_milliseconds_count
    })
}

fn system_tick_exception_handler(
    #[allow(non_snake_case)] current_past_milliseconds_count: &mut u32,
) {
    *current_past_milliseconds_count += 1;
    //
    #[cfg(feature = "enable-debug")]
    {
        let seconds_passed = (*current_past_milliseconds_count / 1000) as u32;
        if *current_past_milliseconds_count % 1000 == 0 {
            hprintln!("New version, seconds_passed: {}", seconds_passed);
        }
    }
}
