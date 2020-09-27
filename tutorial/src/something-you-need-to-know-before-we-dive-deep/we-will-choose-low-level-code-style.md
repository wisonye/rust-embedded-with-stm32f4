### <a name="we-will-choose-low-level-code-style">5.7 Why we pick low-level</a>

_**For the rest of the chapters, we pick the low-level (pure register controlling) code style for all demos, why?**_

No matter what knowledge we’re learning, usually 2 steps we can follow:

_1. We only need to know how to use it. For example, turning on a light bulb or turning it off, that’s simple and fit the requirement as a normal user._

_2. We need to know how it works, then we’re able to change it, even improve it. For example, if you're an energy-saving light bulb company, then you have to figure out how the light bulb works and improve it, that’s another story._

That means there isn't an absolute right answer, it’s totally up to you, as you know what you want. But **we will pick the No.2 way to continue learning and use low-level register controlling style to code for the rest of demos**. Why?

- If you know how this works, then no more secrets you don't know,  you're able to build anything you want. And you should have the ability to work on professional `STM32` project in Rust.

- When you're working on your unique project, you got full control. How to say that? 

    - The `PAC` or `HAL` crate both are designed for generic use cases, may not suit your unique hardware product situation. Sometimes, you found that it's pretty not straightforward to reach what you want based on those crates.

    - Those crates are contributed by many people with different background and skill experience, it will have bugs. When you face that, you can't get fixed immediately as that's out of your control. For example, you can't ask for somebody to fix the bug you encountered as soon as possible (as you need that be fixed on your hardware product). That does not make sense for a generic purpose share library.

So that's why we should figure out the theory and the low-level implementation, then we got the benefits below:

_1. We only rely on the STM official reference manual._

_2. We got full control for picking the easy way and straight ahead solution to code._

_3. We can apply immediate fix or improvement idea or solution at any given time._

_4. After we got real experience, we can build our own `PAC` or `HAL` crate and contribute back to the community._
