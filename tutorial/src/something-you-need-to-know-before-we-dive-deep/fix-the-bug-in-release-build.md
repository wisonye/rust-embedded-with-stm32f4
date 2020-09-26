### <a name="fix-the-bug-in-release-build">5.5 Let's fix the bug in release build</a>

Ok, so far, it works well in the **debug** mode. But actually it won't work like what we expected in the **release** mode.

Let's give it a try by running the commands below:

```bash
# Build and strip the release mode binary
cargo-strip --target thumbv7em-none-eabi --bin gpio_led_by_register --release

# Run that release mode binary in `QEMU`
qemu-system-gnuarmeclipse -cpu cortex-m4 -mcu STM32F407VG -machine STM32F4-Discovery -semihosting-config enable=on,target=native -kernel target/thumbv7em-none-eabi/release/gpio_led_by_register
```

And you should see **all LEDs are off and never on!!!** Wow, what happened there???

Let's take a look at the special lines below:

```rust
60:        *gpiod_moder_mut_ptr |= (1 << 24) | (1 << 26) | (1 << 28) | (1 << 30);

84:        *gpiod_bsrr_mut_ptr = (1 << 12) | (1 << 13) | (1 << 14) | (1 << 15);

95:        *gpiod_bsrr_mut_ptr = (1 << (12 + 16)) | (1 << (13 + 16));
```

For explaining the **potential bug** there, plz have a look the sample code below:

```rust
fn main() {
    let mut a = 10u8;
    println!("a: {}", a);

    let ptr_to_a = &mut a as *mut u8;
    unsafe {
        *ptr_to_a = 50u8;
        *ptr_to_a = 100u8;
        *ptr_to_a = 200u8;
        println!("a: {}", a);
    }
}
```

When building with `--release`, `LLVM` tries to optimize the code. As we assigned three times to the same memory which `ptr_to_a` points to, then `LLVM` may think and see the code like this:


```rust
let ptr_to_a = &mut a as *mut u8;
unsafe {
    // *ptr_to_a = 50u8;
    // *ptr_to_a = 100u8;
    *ptr_to_a = 200u8;
    println!("a: {}", a);
}
```

That's why our `*gpiod_moder_mut_ptr` be assigned with the `unexpected` value and caused the code work not correct.

So, how to fix it? That's easy, use [`core::ptr::write_volatile()`](https://doc.rust-lang.org/stable/core/ptr/fn.write_volatile.html) and [`core::ptr::read_volatile()`](https://doc.rust-lang.org/stable/core/ptr/fn.read_volatile.html) when we deal with register pointer:

```rust

60:     core::ptr::write_volatile(gpiod_moder_mut_ptr, (1 << 24) | (1 << 26) | (1 << 28) | (1 << 30));

84:     core::ptr::write_volatile(gpiod_bsrr_mut_ptr, (1 << 12) | (1 << 13) | (1 << 14) | (1 << 15));

95:     core::ptr::write_volatile(gpiod_bsrr_mut_ptr, (1 << (12 + 16)) | (1 << (13 + 16)));
```

The fixed version is in [gpio_led_by_register_fixed.rs](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/bin/gpio_led_by_register_fixed.rs)

Try it right now, it should work as we expected:

```bash
cargo-strip --target thumbv7em-none-eabi --bin gpio_led_by_register_fixed --release
qemu-system-gnuarmeclipse -cpu cortex-m4 -mcu STM32F407VG -machine STM32F4-Discovery -semihosting-config enable=on,target=native -kernel target/thumbv7em-none-eabi/release/gpio_led_by_register_fixed
```

