#### <a name="install-tooling">3.1 Install tooling</a>

- `rust` and related binaries
    
    ```bash
    # Install latest rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # `cargo-generate` used to generate the rust embedded project from template
    cargo install cargo-generate

    # `cargo-binutils` is a collection of cargo subcommand LLVM tools. Like:
    # `rust-objdump`,
    # `rust-readob`,
    # `rust-ld`,
    # `rust-lld`,
    # `rust-lld`,
    # `rust-nm`,
    # `rust-size`,
    # etc.
    cargo install cargo-binutils

    rustup component add llvm-tools-preview

    # We need this for hot-load
    cargo install cargo-watch

    # Add cross-compilation target
    rustup target add thumbv7em-none-eabi
    rustup target add thumbv7em-none-eabihf
    ```

- `QEMU` ARM emulator and `STM32F4` support

    ```bash
    brew install qemu
    ```

    `QEMU` is a generic and open source machine emulator. By default, it doesn't
    support `STM32F4`, below are the steps how to make that happen:

    ```bash
    # Detail is here: https://xpack.github.io/qemu-arm/install/
    cd ~/Downloads/
    curl -LJO "https://github.com/xpack-dev-tools/qemu-arm-xpack/releases/download/v2.8.0-9/xpack-qemu-arm-2.8.0-9-darwin-x64.tar.gz"
    mkdir -p ~/Library/xPacks/qemu-arm/2.8.0-9
    cd ~/Library/xPacks/qemu-arm/2.8.0-9
    tar xvf ~/Downloads/xpack-qemu-arm-2.8.0-9-darwin-x64.tar.gz
    mv xpack-qemu-arm-2.8.0-9/* ./
    rm -rf xpack-qemu-arm-2.8.0-9/
    chmod -R -w ./

    # Add `/Users/wison/Library/xPacks/qemu-arm/2.8.0-9/bin` to your `$PATH`
    # After that, test it:
    qemu-system-gnuarmeclipse --version

    # List all support machine and we should be able to see `STM32F4XXX`:
    qemu-system-gnuarmeclipse -machine help
    ```

- ARM version `GDB` and `OpenOCD` for remote debugging

    `GDB` isn't able to communicate directly with the ST-Link debugging hardware on your STM32F3DISCOVERY development board. It needs a translator and the Open On-Chip Debugger, `OpenOCD`, is that translator. `OpenOCD` is a program that runs on your laptop/PC and translates between GDB's TCP/IP based remote debug protocol and ST-Link's USB based protocol.

    `OpenOCD` also performs other important work as part of its translation for the debugging of the ARM Cortex-M based microcontroller on your STM32F3DISCOVERY development board:

    It knows how to interact with the memory mapped registers used by the ARM CoreSight debug peripheral. It is these CoreSight registers that allow for:
    Breakpoint/Watchpoint manipulation
    Reading and writing of the CPU registers
    Detecting when the CPU has been halted for a debug event
    Continuing CPU execution after a debug event has been encountered
    etc.
    It also knows how to erase and write to the microcontroller's FLASH

    ```bash
    brew install armmbed/formulae/arm-none-eabi-gcc
    brew install openocd
    ```

    After that, you will find the `OpenOCD` interface configuration and target configuration at here:

    _`/usr/local/Cellar/open-ocd/{VERSION_NUMBER}/share/openocd/scripts/interface/stlink-v2-1.cfg`_
    _`/usr/local/Cellar/open-ocd/{VERSION_NUMBER}/share/openocd/scripts/target/stm32f4x.cfg`_

- **ST-LINK** to use burn your program into `STM32F4` hardware

    ```bash
    brew install stlink
    ```

</br>

[More detail here](https://rust-embedded.github.io/book/intro/tooling.html)
