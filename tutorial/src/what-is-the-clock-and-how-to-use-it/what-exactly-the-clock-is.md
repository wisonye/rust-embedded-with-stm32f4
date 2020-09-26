### <a name="what-exactly-the-clock-is">6.3 What exactly the `Clock` is?</a>

The `Clock` we're talking about actually is `Oscillator`. If you google it or go to the wiki about `oscillator` (RC oscillator, crystal oscillator), it just a circuit, for example, it may look like below:

![oscillator-circuit](../../images/oscillator-circuit.png)

The physical look & feel will look like this:

![oscillator](../../images/oscillator.png)

Yes, that's the `Clock` we're talking about. The only thing you need to keep in mind is that: **That oscillator can generate a stable clock signal to drive all components and peripherals to work correctly.** 

You still can code your program without fully understand what that means, then no worry at this moment:)

Two major `Clock` type in the [reference manual](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/stm32f4-reference-manual.pdf):

- Internal: it means the oscillator circuit is inside the `STM32` chip.
- External: it means the oscillator component is outside the `STM32` chip, usually soldered onboard.
