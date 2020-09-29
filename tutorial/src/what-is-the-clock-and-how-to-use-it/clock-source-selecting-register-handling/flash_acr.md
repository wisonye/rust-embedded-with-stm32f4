#### <a name="flash_acr">6.6.3 Flash access control register (`FLASH_ACR`)</a>

[`reference manual`](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/stm32f4-reference-manual.pdf) page 80:

![flash_read_latency.png](../../images/flash_read_latency.png)

First thing first, we need to know _Why we have to set this flash prefetch latency_?

The explanation above is the answer: _**There is a special relation between CPU clock frequency and Flash memory read time**_.

Then we have to make it right after we change the **`HCLK`** frequency!!!

</br>

So, here is the register info in page 65 and 98:

![flash_register_address.png](../../images/flash_register_address.png)

![flash_acr.png](../../images/flash_acr.png)

What information we got from this diagram?

- `bit0 ~ bit2` controls the flash prefetch latency setting:

    ![flash_acr_latency_setting.png](../../images/flash_acr_latency_setting.png)

    Tips for picking the correct latency value:

    _After you changed the **`HCLK`** frequency in `STMCubeMX` UI,
    back to the `Pinout & Configuration` tab, click on `RCC` on the left side, then you're able to see the correct latency
    setting for the particular frequency like below:_

    ![flash_latency_for_168mhz.png](../../images/flash_latency_for_168mhz.png)

- `bit8` needs to set to `1` for enabling the prefetch.

- `bit9` needs to set to `1` for enabling the instruction cache.

- `bit10` needs to set to `1` for enabling the data cache.

</br>

Here is the source: [demo/src/register_utils/flash_access_control_register.rs](https://github.com/wisonye/rust-embedded-with-stm32f4/blob/master/demo/src/register_utils/flash_access_control_register.rs)

When the final demo runs, the `FLASH_ACR` register debug info will look like below:

![flash_acr_debug.png](../../images/flash_acr_debug.png)
