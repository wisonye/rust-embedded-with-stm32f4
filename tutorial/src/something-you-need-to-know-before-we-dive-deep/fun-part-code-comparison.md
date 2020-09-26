### <a name="fun-part-code-comparison">5.6 The fun part, code comparison</a>

As maybe some of you are wondering which `coding solution` is better: The `HAL` one? or the `Low-level` one?

Let's make a code comparison to have a look (left-side is `HAL`, right-side is `Low-level`):

![code-comparison](../../images/code-compare.png)

- `HAL` pros and cons:
    - It looks like less code (total lines for the same purpose).
    - It can fit all `STM32` series, just change the `features` in `Cargo.toml`.
    - But of course, you need to spent more time to learn the `HAL` crate and get familiar with it (concepts, structs, modules and functions).

</br>

- `Low-level` pros and cons:
    - Sometimes a little more code to setup the registers.
    - Not guaranteed can fit for all `STM32` series (actually, that's not possible).
    - It’s simple and good for hardware background developer: Just open the reference manual and check the particular register, then start to code. All you needed just the basic computer knowledge: bit wise operation.
    - As no more `PAC` or `HAL` needed, then you got full-control and output the binary size as small as possible.

_So that means no right answer, it totally depends on **YOU:)**_
