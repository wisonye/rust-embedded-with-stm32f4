use crate::rcc_clock_settings::{clock_source_selecting, RCC_CR};
use core::ptr;
use cortex_m::asm;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

// ------ SysTick Timer Register (STK) ------------------------
pub const STK_CTRL: u32 = 0xE000E010; // page 246
pub const STK_LOAD: u32 = 0xE000E014; // page 246
pub const STK_VAL: u32 = 0xE000E018; // page 246

pub const STK_CTRL_ENABLE_START_BIT: u8 = 0;
pub const STK_CTRL_ENABLE_BIT: u32 = 1;

pub const STK_CTRL_EXCEPTION_REQUEST_ENABLE_START_BIT: u8 = 1;
pub const STK_CTRL_EXCEPTION_REQUEST_ENABLE_BIT: u32 = 1 << 1;

pub const STK_CTRL_USE_CPU_CLOCK_FREQUENCY_START_BIT: u8 = 2;
pub const STK_CTRL_USE_CPU_CLOCK_FREQUENCY_BIT: u32 = 1 << 2;

pub const STK_CTRL_COUNTDOWN_TO_ZERO_START_BIT: u8 = 16;
pub const STK_CTRL_COUNTDOWN_TO_ZERO_BIT: u32 = 1 << 16;

pub struct SystemTickTimer {}

///
impl SystemTickTimer {
    /// Here is how system tick timer works:
    ///
    /// 1. Set the reload value to `STK_LOAD` register which the value countdown to 0 takes 1
    ///    milliseconds.
    ///
    /// 2. Set the correct clock source to `STK_CTRL` register `bit2`.
    ///
    /// 3. Enable the countdown by setting `1` to `STK_CTRL` register `bit0`, then timer starts
    ///    to countdown and will set `STK_CTRL` register `bit16` to `1` when countdown to 0.
    ///
    /// 4. Write a value to `STL_VAL` register will clear it's current value to `0` and reset
    ///    `STK_CTRL` register `bit16` to `0`. It works like a reset countdown trigger.
    pub fn enable(cpu_clock_frequency_in_hertz: u32, enable_exception: bool) {
        let stk_ctrl_write_ptr = STK_CTRL as *mut u32;
        let stk_reload_write_ptr = STK_LOAD as *mut u32;
        let stk_val_write_ptr = STK_VAL as *mut u32;

        // `1mhz` means got 1_000_000 system ticks per second or 1_000 system ticks per milliseconds.
        //
        // so `cpu_clock_frequency_in_hertz` means got `cpu_clock_frequency_in_hertz / 1000`
        // system ticks per milliseconds.
        //
        // then, for waiting `n` milliseconds, the reload countdown value should be:
        //
        // n * (cpu_clock_frequency_in_hertz / 1000)
        //
        // but can't we do like this, as the `stk_val` register only got 24 bit to save the current
        // countdown value which means will overflow. that's why we only can save `1ms` ticks
        // value and run `n` ms in a loop to reach the goal: wait for n milliseconds

        //
        // and page 248 tells us that we need to do ` (cpu_clock_frequency_in_hertz / 1000) - 1`
        let reload_countdown = (cpu_clock_frequency_in_hertz / 1000) - 1;
        let set_bits = if enable_exception {
            STK_CTRL_USE_CPU_CLOCK_FREQUENCY_BIT
                | STK_CTRL_EXCEPTION_REQUEST_ENABLE_BIT
                | STK_CTRL_ENABLE_BIT
        } else {
            STK_CTRL_USE_CPU_CLOCK_FREQUENCY_BIT | STK_CTRL_ENABLE_BIT
        };
        unsafe {
            ptr::write_volatile(stk_reload_write_ptr, reload_countdown);
            ptr::write_volatile(stk_val_write_ptr, 0x00000000);
            ptr::write_volatile(stk_ctrl_write_ptr as *mut u32, set_bits);
        }
    }

    ///
    pub fn get_current_countdown_value() -> u32 {
        let stk_val_read_ptr = STK_VAL as *const u32;
        unsafe { ptr::read_volatile(stk_val_read_ptr) }
    }

    #[cfg(feature = "enable-debug")]
    pub fn print_config() {
        let stk_ctrl_read_ptr = STK_CTRL as *const u32;
        let stk_ctrl_register_value = unsafe { ptr::read_volatile(stk_ctrl_read_ptr) };

        let enable_bit =
            (stk_ctrl_register_value & STK_CTRL_ENABLE_BIT) >> STK_CTRL_ENABLE_START_BIT;
        let control_register_enabled = enable_bit == 1;

        let exception_request_enabled_bit = (stk_ctrl_register_value
            & STK_CTRL_EXCEPTION_REQUEST_ENABLE_BIT)
            >> STK_CTRL_EXCEPTION_REQUEST_ENABLE_START_BIT;
        let exception_request_enabled = exception_request_enabled_bit == 1;

        let use_cpu_clock_frequency_bit = (stk_ctrl_register_value
            & STK_CTRL_USE_CPU_CLOCK_FREQUENCY_BIT)
            >> STK_CTRL_USE_CPU_CLOCK_FREQUENCY_START_BIT;
        let use_cpu_clock_frequency = use_cpu_clock_frequency_bit == 1;
        let clock_source_desc = if use_cpu_clock_frequency {
            "Processor clock (AHB)"
        } else {
            "AHB/8"
        };

        let countdown_to_zero_bit = (stk_ctrl_register_value & STK_CTRL_COUNTDOWN_TO_ZERO_BIT)
            >> STK_CTRL_COUNTDOWN_TO_ZERO_START_BIT;
        let countdown_to_zero = countdown_to_zero_bit == 1;

        let stk_load_read_ptr = STK_LOAD as *const u32;
        let stk_load_register_value = unsafe { ptr::read_volatile(stk_load_read_ptr) };

        let reload_value = stk_load_register_value;

        let printing_header = "\n[ System Tick Timer Control Register (STK_CTRL) ]: \n";
        let printing_header_2 = "\n\n[ System Tick Timer Reload Register (STK_LOAD) ]: \n";
        let _ = hprintln!(
            "{}{}{}{}{}{}{}{}{}",
            printing_header,
            format_args!("value: {:034b}", stk_ctrl_register_value),
            format_args!(
                "\nCounter enabled: {:?}\t\t\t\t\t\t\t // bits: {:#03b}",
                control_register_enabled, enable_bit
            ),
            format_args!(
                "\nCounting down to zero to asserts the SysTick exception request: {:?}\t // bits: {:#03b}",
                exception_request_enabled, exception_request_enabled_bit
            ),
            format_args!(
                "\nClock source: {:?}\t\t\t\t\t // bits: {:#03b}",
                clock_source_desc, use_cpu_clock_frequency_bit
            ),
            format_args!(
                "\nTimer counted to 0: {:?}\t\t\t\t\t\t // bits: {:#03b}",
                countdown_to_zero, countdown_to_zero_bit
            ),
            printing_header_2,
            format_args!("value: {:034b}", stk_load_register_value),
            format_args!(
                "\nReload value: {:?}\t // bits: {:#026b}",
                reload_value, reload_value
            ),
        );
    }
}
