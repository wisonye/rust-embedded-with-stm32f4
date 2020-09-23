#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry;

#[cfg(feature = "enable-debug")]
use cortex_m_semihosting::hprintln;

use panic_semihosting as _;

// As we don't use `PAC` and `HAL` in this example, and we didn't touch the `Clock` and
// `Interrupt` yet. That's why we use a dumb version `delay` at this moment. It's not
// accuracy, that's fine, as that's not the point we focus on at this moment.
fn dumb_delay(millisecond: u32) {
    delay(100_000 * millisecond);
}

#[entry]
fn main() -> ! {
    #[cfg(feature = "enable-debug")]
    let _ = hprintln!("STM32F4 GPIO Register Led demo is running >>>>>");

    // Below is the very important step:
    //
    // When you first turn on the `MCU`, everything turns off for power saving. We need to enable
    // the `GPIOD` port. Info in `reference manual` (page 265, RCC register map).
    //
    // RCC (Reset and Clock Control)
    const RCC_REGISTER: u32 = 0x4002_3800;
    const RCC_AHB1ENR_REGISTER: u32 = RCC_REGISTER + 0x30; // page 242, 243
    const RCC_AHB1LPENR_REGISTER: u32 = RCC_REGISTER + 0x50; // Low power (sleep) mode, page 250, 252,
    unsafe {
        // Enable `GPIOD` by setting the `bit3` to `1` for both registers.
        *(RCC_AHB1ENR_REGISTER as *mut u32) = 1 << 3;
        *(RCC_AHB1LPENR_REGISTER as *mut u32) = 1 << 3;
    }

    // `GPIOD` register mapping address, in `reference manual` (page 65, `STM32F4xx register boundary addresses`).
    const GPIOD_REGISTER: u32 = 0x4002_0c00;

    // GPIO port mode register (GPIOx_MODER) address, `reference manual` (page 281).
    const GPIOD_MODER: u32 = GPIOD_REGISTER + 0x00;
    let gpiod_moder_mut_ptr: *mut u32 = GPIOD_MODER as *mut u32; // Mutable raw pointer
    let gpiod_moder_ptr: *const u32 = GPIOD_MODER as *const u32; // Immutable raw pointer
    unsafe {
        // Set `GPIOD` pin12 ~ pin15 to OUTPUT mode

        // Keep the prev value and add (`|`) new setting:
        // bit 25, 24 set to `01`
        // bit 27, 26 set to `01`
        // bit 29, 28 set to `01`
        // bit 31, 30 set to `01`
        //
        // As the "GPIOD_BSRR" does nothing when set bit to `0`, so actually, we even don't
        // need the `|=` for keeping the prev value. But we keep that just doing in the normal
        // way.
        //
        *gpiod_moder_mut_ptr |= (1 << 24) | (1 << 26) | (1 << 28) | (1 << 30);

        // Let's print the "GPIOD_MODER" register bit value (32bit, 4 bytes), and it should be:
        // 0b01010101000000000000000000000000
        //
        // From right to left is bit0 ~ bit31, only bit24, bit26, bit 28, bit30 set to `1`.
        #[cfg(feature = "enable-debug")]
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
        // Set bit (pin) 12 ~ 15 to `1` to turn on 4 LEDs.
        //
        // As the "GPIOD_BSRR" does nothing when setting bit to `0`, so actually, we even don't
        // need the `|=` for keeping the previous value.
        *gpiod_bsrr_mut_ptr = (1 << 12) | (1 << 13) | (1 << 14) | (1 << 15);
    }

    #[cfg(feature = "enable-debug")]
    let _ = hprintln!("\nDelay 1s......\n");

    dumb_delay(10000);

    unsafe {
        // Set bit (pint) 12 + 16, 13 + 16 to `1` to turn off 2 LEDs.
        //
        // As the "GPIOD_BSRR" does nothing when setting bit to `0`, so actually, we even don't
        // need the `|=` for keeping the previous value.
        *gpiod_bsrr_mut_ptr = (1 << (12 + 16)) | (1 << (13 + 16));
    }

    loop {}
}
