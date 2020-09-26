### <a name="mcu-gpio-register">5.2 STM32F4 **GPIOP** registers</a>

This particular register controls **G**eneral **P**urpose **I**nput/**O**utput (**GPIO**) pins (**GPIO** is a peripheral) and can be used to drive each of those pins **low** or **high**:

- **Low** means the particular pin is **0** voltage.
- **Hight** means the particular pin is **output** voltage. For example **5v** or **3.3v** which depends on your `MCU`.


Let's take a look at the **GPIO** registers in the [`reference manual`](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/stm32f4-reference-manual.pdf) page **267**:

Each **GPIO** port has ten reigsters:

- Four 32-bit configuration registers (**GPIOx_MODER, GPIOx_OTYPER, GPIOx_OSPEEDR and GPIOx_PUPDR**).
- Two 32-bit data registers (**GPIOx_IDR and GPIOx_ODR**). 
- A 32-bit set/reset register (**GPIOx_BSRR**). 
- A 32-bit locking register (**GPIOx_LCKR**).
- Two 32-bit alternate function selection register (**GPIOx_AFRH and GPIOx_AFRL**).

**x** means the port number, each **GPIO** port can handle 16 pins (pin0 ~ pin15). How many **GPIO** port (the **x** value) you have totally depended on the `MCU` itself. For `STM32F407VG` has `GPIOA ~ GPIOK` (11 ports).

It sounds crazy, but let's only focus on the registers below:

- **GPIOx_MODER**: Modify register, it‘s responsible for setting the GPIO pin to **output** or **input** mode.
- **GPIOx_OTYPER**: Output type register, it's responsible for setting the GPIO pin to specified type: **PushPull, OpenDrain**.
- **GPIOx_IDR**: Input data register, it's responsible for reading data from the GPIO pin.
- **GPIOx_ODR**: Output data register, it's responsible for writing data to the GPIO pin. _But we don't use it, we use **GPIOx_BSRR** instead, that's an atomic write operation._
- **GPIOx_BSRR**: Bit set/reset register, use to write **High** or reset to **Low** to the GPIO pin.

