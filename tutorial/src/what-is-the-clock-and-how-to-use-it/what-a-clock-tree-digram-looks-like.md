### <a name="what-a-clock-tree-digram-looks-like">6.4 What a professional `Clock tree diagram` looks like?</a>

The below diagram is a screenshot from STMCubeMX projet, it shows a pretty straightforward strategy of clock source selecting:

![mcu_clock_sources.png](../../images/mcu_clock_sources.png)

Yes, it looks very complicated and scary for the first time, but relax, it's not that hard actually:)

So what information we got from the diagram?   

- Based on the **blue** highlighted, we know that there are 2 external clock sources can connect to the chip for different purposes.

- Based on the **green** highlighted, we know that there are 2 internal clock sources inside the chip for different purposes.

- The **orange** highlighted shows what the final system clock (`SYSCLK`) working frequency will be.

- The **purple** highlighted let us know what exact working frequency on the particular peripherals.

That's it, that's enough for us at this moment.

_Tips:_

- _`STMCubeMX` is a `STMCube` project initialization and code generating tool which you can download [here](https://www.st.com/en/development-tools/stm32cubemx.html)._
- _`STMCubeMX` just an optional tool for us, as it generates the **C++** code, not the **Rust** code. But for the **Clock source selecting**, that diagram is super helpful for us, good to have._
