#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

use crate::hal::{
    prelude::*,
    stm32, // The `stm32` should means `stm32f407` which enable by the `features` in `.toml`
};

// This is very important!!!
use stm32f4xx_hal as hal;

// Import from `stm32f4xx_hal`
use hal::{delay::Delay, rcc::Rcc};

// Set to `false` when u don't need that anymore
const ENABLE_DEBUG: bool = true;

#[entry]
fn main() -> ! {
    if ENABLE_DEBUG {
        let _ = hprintln!("STM32F4 GPIO Register Led demo is running >>>>>");
    }

    let stm32407_peripherals = stm32::Peripherals::take().unwrap();
    let cortex_m_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock. We want to run at 16Mhz for this one.
    let constrained_rcc_peripheral: Rcc = stm32407_peripherals.RCC.constrain();
    let clocks = constrained_rcc_peripheral.cfgr.sysclk(16.mhz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = Delay::new(cortex_m_peripherals.SYST, clocks);

    // I don't know how the `sysclk` works and how to set the correct `Mhz`, but for now,
    // the `excepted_delay_time_in_ms` needs to cut half for getting the correct delay time.
    let excepted_delay_time_in_ms = 1000u32;
    let delay_time_in_ms = (excepted_delay_time_in_ms / 2) as u32;

    // Below is the very important step:
    //
    // When you first turn on the `MCU`, everything turns off for power saving. We need to enable
    // the `GPIOD` port. Info in `reference manual` (page 265, RCC register map).
    //
    // RCC (Reset and Clock Control)
    const RCC_REGISTER: u32 = 0x40023800;
    const RCC_AHB1ENR_REGISTER: u32 = RCC_REGISTER + 0x30; // page 242, 243
    const RCC_AHB1LPENR_REGISTER: u32 = RCC_REGISTER + 0x50; // Low power (sleep) mode, page 250, 252,
    unsafe {
        // Enable `GPIOD` at bit 3
        *(RCC_AHB1ENR_REGISTER as *mut u32) = 1 << 3;
        *(RCC_AHB1LPENR_REGISTER as *mut u32) = 1 << 3;
    }

    // `GPIOD` register mapping address, in `reference manual` (page 65, `STM32F4xx register boundary addresses`).
    const GPIOD_REGISTER: u32 = 0x40020c00;

    // GPIO port mode register (GPIOx_MODER) address, `reference manual` (page 281).
    const GPIOD_MODER: u32 = GPIOD_REGISTER + 0x00;
    let gpiod_moder_mut_ptr: *mut u32 = GPIOD_MODER as *mut u32; // Mutable raw pointer
    let gpiod_moder_ptr: *const u32 = GPIOD_MODER as *const u32; // Immutable raw pointer
    unsafe {
        // Set `GPIOD` pin12 ~ pin15 to OUTPUT mode

        // bit 25, 24 set to `01`
        *gpiod_moder_mut_ptr = 1 << 24; 

        // Keep the prev value and add (`|`) new setting: bit 27, 26 set to `01`
        *gpiod_moder_mut_ptr = *gpiod_moder_ptr | (1 << 26); 

        // Keep the prev value and add (`|`) new setting: bit 29, 28 set to `01`
        *gpiod_moder_mut_ptr = *gpiod_moder_ptr | (1 << 28); 

        // Keep the prev value and add (`|`) new setting: bit 31, 30 set to `01`
        *gpiod_moder_mut_ptr = *gpiod_moder_ptr | (1 << 30); 

        // Let's print the "GPIOD_MODER" register bit value (32bit, 4 bytes), and it should be:
        // 0b01010101000000000000000000000000
        // 
        // From right to left is bit0 ~ bit31, only bit24, bit26, bit 28, bit30 set to `1`.
        let _ = hprintln!("GPIOD_MODER: {:#034b}", *gpiod_moder_ptr);
    }

    // GPIO port output type register (GPIOx_OTYPER) address, `reference manual` (page 281).
    // As the output type `push-pull` is `0`, then we don't need to set `GPIOD_OTYPER` explicitly.
    // const GPIOD_OTYPER: u32 = GPIOD_REGISTER + 0x04;

    // GPIO port bit set/reset register (GPIOx_BSRR) address, `reference manual` (page 284).
    const GPIOD_BSRR: u32 = GPIOD_REGISTER + 0x18;
    let gpiod_bsrr_mut_ptr = GPIOD_BSRR as *mut u32;

    // Setup GPIOD.P12 ~ P15 to output mode with `

    unsafe {
        // Set bit (pin) 12 ~ 15 to `1` to turn on 4 LEDs. As the "GPIOD_BSRR" does nothing when
        // set bit to `0`, that's why we don't need to `|` the prev register value:)
        *gpiod_bsrr_mut_ptr = 1 << 12;
        *gpiod_bsrr_mut_ptr = 1 << 13;
        *gpiod_bsrr_mut_ptr = 1 << 14;
        *gpiod_bsrr_mut_ptr = 1 << 15;
    }

    let _ = hprintln!("\nDelay 1s......\n");
    delay.delay_ms(delay_time_in_ms);

    unsafe {
        // Set bit (pint) 12 + 16, 13 + 16 to `1` to turn off 2 LEDs. As the "GPIOD_BSRR" does nothing when
        // set bit to `0`, that's why we don't need to `|` the prev register value:)
        *gpiod_bsrr_mut_ptr = 1 << (12 + 16);
        *gpiod_bsrr_mut_ptr = 1 << (13 + 16);
        // *gpiod_bsrr_mut_ptr = 1 << (14 + 16);
        // *gpiod_bsrr_mut_ptr = 1 << (15 + 16);
    }

    loop {}
}
